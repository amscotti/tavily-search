pub mod api;
pub mod cli;
pub mod error;
pub mod output;

pub use api::ExtractedContent;
pub use api::TavilyClient;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
