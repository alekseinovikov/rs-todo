pub mod service_impl;

use crate::task::{ShortTask, Task};
use thiserror::Error;

pub struct CreateTaskDto {
    pub title: String,
    pub description: String,
}

pub struct UpdateTaskDto {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("`{0}`")]
    StorageError(String),
}

pub trait Service {
    fn get_all(&self) -> Result<Vec<ShortTask>, ServiceError>;
    fn get_all_undone(&self) -> Result<Vec<ShortTask>, ServiceError>;
    fn get_all_done(&self) -> Result<Vec<ShortTask>, ServiceError>;
    fn get(&self, id: i32) -> Result<Option<Task>, ServiceError>;
    fn create(&self, task: CreateTaskDto) -> Result<Task, ServiceError>;
    fn update(&self, id: i32, task: UpdateTaskDto) -> Result<Option<Task>, ServiceError>;
    fn mark_done(&self, id: i32) -> Result<Option<Task>, ServiceError>;
}
