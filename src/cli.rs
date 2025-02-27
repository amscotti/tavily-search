use clap::{Parser, Subcommand};

/// A CLI for searching the web using Tavily API
#[derive(Parser, Debug)]
#[command(name = "tavily-search")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The search query
    #[arg(index = 1)]
    pub query: Option<String>,

    /// Disable AI-generated answer (enabled by default)
    #[arg(short = 'A', long = "no-answer")]
    pub no_answer: bool,

    /// Search depth (basic or advanced)
    #[arg(short, long, default_value = "basic")]
    pub depth: String,

    /// Maximum number of results
    #[arg(short, long, default_value = "5")]
    pub max_results: u8,

    /// Automatically extract content from top N search results
    #[arg(short = 'e', long, value_name = "N")]
    pub extract_top: Option<u8>,

    /// Maximum characters to display for extracted content
    #[arg(long, default_value = "800")]
    pub extract_limit: usize,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Extract content from URLs
    Extract {
        /// URLs to extract content from
        urls: Vec<String>,

        /// Maximum characters to display for extracted content
        #[arg(long, default_value = "800")]
        extract_limit: usize,
    },
}
