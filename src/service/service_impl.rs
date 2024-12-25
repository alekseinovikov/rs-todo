use crate::service::{CreateTaskDto, Service, UpdateTaskDto};
use crate::storage::Storage;
use crate::task::Task;

pub struct ServiceImpl {
    storage: Box<dyn Storage>,
}

impl Service for ServiceImpl {
    fn get_all(&self) -> Vec<Task> {
        todo!()
    }

    fn get_all_undone(&self) -> Vec<Task> {
        todo!()
    }

    fn get_all_done(&self) -> Vec<Task> {
        todo!()
    }

    fn get(&self, id: i32) -> Option<Task> {
        todo!()
    }

    fn create(&self, task: CreateTaskDto) -> Task {
        todo!()
    }

    fn update(&self, id: i32, task: UpdateTaskDto) -> Option<Task> {
        todo!()
    }

    fn mark_done(&self, id: i32) -> Option<Task> {
        todo!()
    }
}
