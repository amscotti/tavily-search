use tavily_search::ExtractedContent;

#[test]
fn test_extracted_content_creation() {
    let url = "https://example.com";
    let title = "Example Title";
    let content = "Example content text";

    let extracted = ExtractedContent {
        url: url.to_string(),
        title: Some(title.to_string()),
        content: Some(content.to_string()),
    };

    assert_eq!(extracted.url, url);
    assert_eq!(extracted.title.unwrap(), title);
    assert_eq!(extracted.content.unwrap(), content);
}

#[test]
fn test_extracted_content_without_title() {
    let url = "https://example.com";
    let content = "Example content text";

    let extracted = ExtractedContent {
        url: url.to_string(),
        title: None,
        content: Some(content.to_string()),
    };

    assert_eq!(extracted.url, url);
    assert!(extracted.title.is_none());
    assert_eq!(extracted.content.unwrap(), content);
}

#[test]
fn test_extracted_content_without_content() {
    let url = "https://example.com";
    let title = "Example Title";

    let extracted = ExtractedContent {
        url: url.to_string(),
        title: Some(title.to_string()),
        content: None,
    };

    assert_eq!(extracted.url, url);
    assert_eq!(extracted.title.unwrap(), title);
    assert!(extracted.content.is_none());
}
