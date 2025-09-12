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
    pub fn internal<T: Into<String>>(message: T) -> Self {
        Error::Internal {
            message: message.into(),
        }
    }

    pub fn not_found<T: Into<String>>(message: T) -> Self {
        Error::NotFound {
            message: message.into(),
        }
    }

    pub fn invalid_args<T: Into<String>>(message: T) -> Self {
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
