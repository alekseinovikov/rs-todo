use crate::storage::{Storage, StorageError, StorageInit};
use crate::task::Task;
use rusqlite::{Connection, Error};
use std::path::Path;

pub struct StorageSqlite {
    connection: Connection,
}

impl From<Error> for StorageError {
    fn from(err: Error) -> StorageError {
        StorageError::DatabaseError(format!("error on database operation: {}", err))
    }
}

impl StorageSqlite {
    pub fn new(file_path: &str) -> Result<StorageSqlite, StorageError> {
        let connection = Connection::open(Path::new(file_path))?;
        Ok(StorageSqlite { connection })
    }
}

impl StorageInit for StorageSqlite {
    fn init(&self) -> Result<(), StorageError> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                done BOOLEAN NOT NULL
            )",
            [],
        )?;
        Ok(())
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
