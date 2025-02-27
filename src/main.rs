use clap::{CommandFactory, Parser};
use colored::*;
use dotenv::dotenv;
use std::env;
use tavily_search::{
    cli::Cli,
    cli::Commands,
    output::{print_extraction_results, print_search_results},
    Error, Result, TavilyClient,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Parse command line arguments
    let cli = Cli::parse();

    // Check if API key is available
    let api_key = env::var("TAVILY_API_KEY")
        .map_err(|_| Error::Environment(
            "TAVILY_API_KEY environment variable not set. Please set it in your environment or in a .env file".to_string()
        ))?;

    // Validate that API key is not empty
    if api_key.is_empty() {
        return Err(Error::Environment("API key cannot be empty".to_string()));
    }

    // If no arguments are provided, show help
    if cli.query.is_none() && cli.command.is_none() {
        Cli::command().print_help()?;
        return Ok(());
    }

    // Create Tavily client
    let client = TavilyClient::new(&api_key)?;

    // Process commands
    match &cli.command {
        Some(Commands::Extract {
            urls,
            extract_limit,
        }) => {
            let extracted_content = client.extract(urls.clone()).await?;
            // No titles available in direct extraction
            print_extraction_results(&extracted_content, *extract_limit);
        }
        None => {
            if let Some(query) = &cli.query {
                // Perform the search (use answer by default unless explicitly disabled)
                let use_answer = !cli.no_answer;
                let response = client
                    .search(query, use_answer, &cli.depth, cli.max_results)
                    .await?;

                // Print search results
                print_search_results(&response);

                // If extract_top is specified, extract content from top N results
                if let Some(top_n) = cli.extract_top {
                    if top_n > 0 {
                        println!(
                            "\n{}",
                            "Extracting content from top search results...".bold()
                        );

                        // Get URLs and titles from top N results
                        let mut urls: Vec<String> = Vec::new();
                        let mut url_to_title = std::collections::HashMap::new();

                        // Collect URLs and their titles
                        for result in response.results.iter().take(top_n as usize) {
                            urls.push(result.url.clone());
                            url_to_title.insert(result.url.clone(), result.title.clone());
                        }

                        if !urls.is_empty() {
                            let mut extracted_content = client.extract(urls).await?;

                            // Add titles to extracted content
                            for content in &mut extracted_content {
                                if let Some(title) = url_to_title.get(&content.url) {
                                    content.title = Some(title.clone());
                                }
                            }

                            println!("\n{}", "Extracted Content:".bold());
                            print_extraction_results(&extracted_content, cli.extract_limit);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
