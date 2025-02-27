use tavily::SearchResponse;
use tavily::SearchResult;
use tavily_search::ExtractedContent;

// This test is more about ensuring the function doesn't panic
// with various inputs rather than testing the actual output format
#[test]
fn test_extract_content_formatting_no_panic() {
    use tavily_search::output::print_extraction_results;

    // Empty results
    let empty_results: Vec<ExtractedContent> = vec![];
    print_extraction_results(&empty_results, 800);

    // Results with all fields
    let results = vec![
        ExtractedContent {
            url: "https://example.com".to_string(),
            title: Some("Example Title".to_string()),
            content: Some("Some content text".to_string()),
        },
        ExtractedContent {
            url: "https://another-example.com".to_string(),
            title: Some("Another Title".to_string()),
            content: Some("Another content text".to_string()),
        },
    ];
    print_extraction_results(&results, 800);

    // Results with missing fields
    let mixed_results = vec![
        ExtractedContent {
            url: "https://example.com".to_string(),
            title: None,
            content: Some("Content without title".to_string()),
        },
        ExtractedContent {
            url: "https://another-example.com".to_string(),
            title: Some("Title without content".to_string()),
            content: None,
        },
    ];
    print_extraction_results(&mixed_results, 800);
}

// Mock for SearchResponse
fn create_mock_search_response() -> SearchResponse {
    SearchResponse {
        query: "test query".to_string(),
        answer: Some("This is a test answer".to_string()),
        results: vec![
            SearchResult {
                title: "Result 1".to_string(),
                url: "https://example.com/1".to_string(),
                content: "Content for result 1".to_string(),
                raw_content: Some("Raw content for result 1".to_string()),
                score: 0.95,
            },
            SearchResult {
                title: "Result 2".to_string(),
                url: "https://example.com/2".to_string(),
                content: "Content for result 2".to_string(),
                raw_content: Some("Raw content for result 2".to_string()),
                score: 0.85,
            },
        ],
        images: None,
        follow_up_questions: Some(vec![
            "Follow-up question 1?".to_string(),
            "Follow-up question 2?".to_string(),
        ]),
        response_time: 0.5,
    }
}

#[test]
fn test_search_results_formatting_no_panic() {
    use tavily_search::output::print_search_results;

    // Test with mock data
    let response = create_mock_search_response();
    print_search_results(&response);

    // Test with empty results
    let empty_response = SearchResponse {
        query: "empty query".to_string(),
        answer: None,
        results: vec![],
        images: None,
        follow_up_questions: None,
        response_time: 0.1,
    };
    print_search_results(&empty_response);
}
