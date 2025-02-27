use clap::Parser;
use tavily_search::cli::{Cli, Commands};

#[test]
fn test_cli_parse_query() {
    let cli = Cli::parse_from(["program", "test query"]);
    assert_eq!(cli.query, Some("test query".to_string()));
    assert_eq!(cli.no_answer, false);
    assert_eq!(cli.depth, "basic");
    assert_eq!(cli.max_results, 5);
    assert!(cli.command.is_none());
}

#[test]
fn test_cli_parse_with_options() {
    let cli = Cli::parse_from([
        "program",
        "--no-answer",
        "--depth",
        "advanced",
        "--max-results",
        "10",
        "test query",
    ]);

    assert_eq!(cli.query, Some("test query".to_string()));
    assert_eq!(cli.no_answer, true);
    assert_eq!(cli.depth, "advanced");
    assert_eq!(cli.max_results, 10);
    assert!(cli.command.is_none());
}

#[test]
fn test_cli_parse_extract_command() {
    let cli = Cli::parse_from([
        "program",
        "extract",
        "https://example.com",
        "https://another-example.com",
    ]);

    assert!(cli.query.is_none());

    match cli.command {
        Some(Commands::Extract {
            urls,
            extract_limit,
        }) => {
            assert_eq!(urls.len(), 2);
            assert_eq!(urls[0], "https://example.com");
            assert_eq!(urls[1], "https://another-example.com");
            assert_eq!(extract_limit, 800);
        }
        _ => panic!("Expected Extract command"),
    }
}

#[test]
fn test_cli_parse_extract_top() {
    let cli = Cli::parse_from(["program", "--extract-top", "3", "test query"]);

    assert_eq!(cli.query, Some("test query".to_string()));
    assert_eq!(cli.extract_top, Some(3));
}

#[test]
fn test_cli_parse_extract_limit() {
    let cli = Cli::parse_from(["program", "--extract-limit", "1000", "test query"]);

    assert_eq!(cli.query, Some("test query".to_string()));
    assert_eq!(cli.extract_limit, 1000);
}
