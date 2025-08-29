/// Project level error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid arguments: {messages:?}")]
    InvalidArgs { messages: Vec<String> },
    #[error("internal error: {message}")]
    Internal { message: String },
    #[error("not found error: {message}")]
    NotFound { message: String },
}

// Error helpers
impl Error {
    pub fn internal(message: String) -> Self {
        Error::Internal { message }
    }

    pub fn not_found(message: String) -> Self {
        Error::NotFound { message }
    }

    pub fn invalid_args(message: &str) -> Self {
        Error::InvalidArgs {
            messages: vec![message.into()],
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::internal(err.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::internal(err.to_string())
    }
}
