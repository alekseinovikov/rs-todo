use crate::storage::Storage;
use crate::task::Task;
use rusqlite::Connection;
use std::path::Path;
use thiserror::Error;

pub struct StorageSqlite {
    connection: Connection,
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("error on database operation: `{0}`")]
    DatabaseError(#[from] rusqlite::Error),
}

impl StorageSqlite {
    pub fn new(file_path: &str) -> Result<StorageSqlite, StorageError> {
        let connection = Connection::open(Path::new(file_path))?;
        Ok(StorageSqlite { connection })
    }
}

impl Storage for StorageSqlite {
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

    fn create(&self, task: Task) -> Task {
        todo!()
    }

    fn update(&self, task: Task) -> Option<Task> {
        todo!()
    }
}
