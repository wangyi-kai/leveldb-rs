use thiserror::Error;

use crate::skiplist::error::SkipListError;

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SkipList(#[from] SkipListError)
}