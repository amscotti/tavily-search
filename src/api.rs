use crate::{Error, Result};
use std::time::Duration;
use tavily::{SearchRequest, SearchResponse, Tavily};

pub struct TavilyClient {
    client: Tavily,
    api_key: String,
}

impl TavilyClient {
    /// Create a new Tavily client with the given API key
    pub fn new(api_key: &str) -> Result<Self> {
        let client = Tavily::builder(api_key)
            .timeout(Duration::from_secs(60))
            .max_retries(3)
            .build()
            .map_err(|e| Error::TavilyApi(e.to_string()))?;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
        })
    }

    /// Perform a search with the given query
    pub async fn search(
        &self,
        query: &str,
        include_answer: bool,
        search_depth: &str,
        max_results: u8,
    ) -> Result<SearchResponse> {
        if include_answer {
            // Use answer method for searches with AI answers
            self.client
                .answer(query)
                .await
                .map_err(|e| Error::TavilyApi(e.to_string()))
        } else {
            // Use custom search request for more options
            let request = SearchRequest::new(&self.api_key, query)
                .search_depth(search_depth)
                .max_results(max_results as i32);

            self.client
                .call(&request)
                .await
                .map_err(|e| Error::TavilyApi(e.to_string()))
        }
    }

    /// Extract content from the given URLs
    pub async fn extract(&self, urls: Vec<String>) -> Result<Vec<ExtractedContent>> {
        self.client
            .extract(urls)
            .await
            .map(|response| {
                response
                    .results
                    .into_iter()
                    .map(|r| ExtractedContent {
                        url: r.url,
                        title: None, // We don't have titles from direct extraction
                        content: Some(r.raw_content),
                    })
                    .collect()
            })
            .map_err(|e| Error::TavilyApi(e.to_string()))
    }
}

/// Simplified structure for extracted content
pub struct ExtractedContent {
    pub url: String,
    pub title: Option<String>,
    pub content: Option<String>,
}
