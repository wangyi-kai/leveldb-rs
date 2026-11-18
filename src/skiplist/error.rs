use thiserror::Error;

#[derive(Debug, Error)]
pub enum SkipListError {
    #[error("{0}")]
    Fatal(String),
    #[error("{0}")]
    NodeErr(String),
    #[error("{0}")]
    IteratorErr(String),
}