use crate::task::{ShortTask, Task};
use std::error::Error;
use thiserror::Error;

pub mod storage_sqlite;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("error on database operation: `{0}`")]
    DatabaseError(String),
}

pub trait Storage {
    fn get_all(&self) -> Result<Vec<ShortTask>, StorageError>;
    fn get_all_undone(&self) -> Result<Vec<ShortTask>, StorageError>;
    fn get_all_done(&self) -> Result<Vec<ShortTask>, StorageError>;
    fn get(&self, id: i32) -> Result<Option<Task>, StorageError>;
    fn create(&self, task: Task) -> Result<Task, StorageError>;
    fn update(&self, task: Task) -> Result<Task, StorageError>;
}

pub trait StorageInit {
    fn init(&self) -> Result<(), StorageError>;
}
