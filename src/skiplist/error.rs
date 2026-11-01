use thiserror::Error;

#[derive(Debug, Error)]
pub enum SkipListError {
    #[error("{0}")]
    Fatal(String),
}