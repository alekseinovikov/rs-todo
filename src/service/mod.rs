mod service_impl;

use crate::task::Task;

pub struct CreateTaskDto {
    pub title: String,
    pub description: String,
}

pub struct UpdateTaskDto {
    pub title: Option<String>,
    pub description: Option<String>,
}

pub trait Service {
    fn get_all(&self) -> Vec<Task>;
    fn get_all_undone(&self) -> Vec<Task>;
    fn get_all_done(&self) -> Vec<Task>;
    fn get(&self, id: i32) -> Option<Task>;
    fn create(&self, task: CreateTaskDto) -> Task;
    fn update(&self, id: i32, task: UpdateTaskDto) -> Option<Task>;
    fn mark_done(&self, id: i32) -> Option<Task>;
}
