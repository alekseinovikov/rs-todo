use crate::task::Task;
use std::error::Error;
use thiserror::Error;

pub mod storage_sqlite;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("error on database operation: `{0}`")]
    DatabaseError(String),
}

pub trait Storage {
    fn get_all(&self) -> Vec<Task>;
    fn get_all_undone(&self) -> Vec<Task>;
    fn get_all_done(&self) -> Vec<Task>;
    fn get(&self, id: i32) -> Option<Task>;
    fn create(&self, task: Task) -> Task;
    fn update(&self, task: Task) -> Option<Task>;
}

pub trait StorageInit {
    fn init(&self) -> Result<(), StorageError>;
}
