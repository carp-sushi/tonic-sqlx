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
    pub fn internal(message: impl Into<String>) -> Self {
        Error::Internal {
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Error::NotFound {
            message: message.into(),
        }
    }

    pub fn invalid_args(message: impl Into<String>) -> Self {
        Error::InvalidArgs {
            messages: vec![message.into()],
        }
    }
}
