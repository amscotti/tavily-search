use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Tavily API error: {0}")]
    TavilyApi(String),

    #[error("Environment error: {0}")]
    Environment(String),

    #[error("Input validation error: {0}")]
    InputValidation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("General error: {0}")]
    General(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::General(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::General(s.to_string())
    }
}
