use crate::service::{CreateTaskDto, Service, ServiceError, UpdateTaskDto};
use crate::storage::{Storage, StorageError, StorageInit};
use crate::task::{ShortTask, Task};
use chrono::Utc;

pub struct ServiceImpl {
    storage: Box<dyn Storage>,
}

impl ServiceImpl {
    pub fn new<T: Storage + StorageInit + 'static>(
        storage: Box<T>,
    ) -> Result<ServiceImpl, ServiceError> {
        storage.init()?;
        Ok(ServiceImpl { storage })
    }
}

impl From<StorageError> for ServiceError {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::DatabaseError(msg) => ServiceError::StorageError(msg),
        }
    }
}

impl Service for ServiceImpl {
    fn get_all(&self) -> Result<Vec<ShortTask>, ServiceError> {
        Ok(self.storage.get_all()?)
    }

    fn get_all_undone(&self) -> Result<Vec<ShortTask>, ServiceError> {
        Ok(self.storage.get_all_undone()?)
    }

    fn get_all_done(&self) -> Result<Vec<ShortTask>, ServiceError> {
        Ok(self.storage.get_all_done()?)
    }

    fn get(&self, id: i32) -> Result<Option<Task>, ServiceError> {
        Ok(self.storage.get(id)?)
    }

    fn create(&self, task: CreateTaskDto) -> Result<Task, ServiceError> {
        Ok(self
            .storage
            .create(Task::new(task.title, task.description))?)
    }

    fn update(&self, id: i32, update_task: UpdateTaskDto) -> Result<Option<Task>, ServiceError> {
        let mut task = self.storage.get(id)?;
        if task.is_none() {
            return Ok(None);
        }
        let mut task = task.unwrap();
        if let Some(title) = update_task.title {
            task.title = title;
        }
        if let Some(description) = update_task.description {
            task.description = description;
        }

        task.updated = Utc::now();
        Ok(Some(self.storage.update(task)?))
    }

    fn mark_done(&self, id: i32) -> Result<Option<Task>, ServiceError> {
        let mut task = self.storage.get(id)?;
        if task.is_none() {
            return Ok(None);
        }

        let mut task = task.unwrap();
        task.done = true;
        task.updated = Utc::now();
        Ok(Some(self.storage.update(task)?))
    }
}
