use crate::task::Task;

pub mod storage_sqlite;

pub trait Storage {
    fn get_all(&self) -> Vec<Task>;
    fn get_all_undone(&self) -> Vec<Task>;
    fn get_all_done(&self) -> Vec<Task>;
    fn get(&self, id: i32) -> Option<Task>;
    fn create(&self, task: Task) -> Task;
    fn update(&self, task: Task) -> Option<Task>;
}
