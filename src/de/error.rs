#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    ValueError(#[from] serde::de::value::Error),
    #[error(transparent)]
    AutomergeError(#[from] automerge::AutomergeError),
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}
