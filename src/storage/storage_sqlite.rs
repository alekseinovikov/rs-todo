use crate::storage::{Storage, StorageError, StorageInit};
use crate::task::{ShortTask, Task};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Error, Row};
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
                created TEXT NOT NULL,
                updated TEXT NOT NULL,
                done BOOLEAN NOT NULL
            )",
            [],
        )?;

        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS ix_tasks_done_created ON tasks (done, updated DESC)",
            [],
        )?;

        Ok(())
    }
}

fn parse_full_row(row: &Row) -> Result<Task, Error> {
    let created_str: String = row.get(3)?;
    let updated_str: String = row.get(4)?;
    Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        created: created_str.parse::<DateTime<Utc>>().unwrap(),
        updated: updated_str.parse::<DateTime<Utc>>().unwrap(),
        done: row.get(5)?,
    })
}

fn parse_short_row(row: &Row) -> Result<ShortTask, Error> {
    let updated_str: String = row.get(2)?;
    Ok(ShortTask {
        id: row.get(0)?,
        title: row.get(1)?,
        updated: updated_str.parse::<DateTime<Utc>>().unwrap(),
        done: row.get(3)?,
    })
}

impl Storage for StorageSqlite {
    fn get_all(&self) -> Result<Vec<ShortTask>, StorageError> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, title, updated, done FROM tasks")?;
        let task_iter = stmt.query_map([], |row| parse_short_row(row))?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    fn get_all_undone(&self) -> Result<Vec<ShortTask>, StorageError> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, updated, done FROM tasks WHERE done = false ORDER BY updated DESC",
        )?;
        let task_iter = stmt.query_map([], |row| parse_short_row(row))?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    fn get_all_done(&self) -> Result<Vec<ShortTask>, StorageError> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, updated, done FROM tasks WHERE done = true ORDER BY updated DESC",
        )?;
        let task_iter = stmt.query_map([], |row| parse_short_row(row))?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    fn get(&self, id: i32) -> Result<Option<Task>, StorageError> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, created, updated, done FROM tasks WHERE id = ?1",
        )?;
        let result = stmt.query_row([id], |row| parse_full_row(row));
        match result {
            Ok(task) => Ok(Some(task)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    fn create(&self, task: Task) -> Result<Task, StorageError> {
        let created_str = task.created.to_rfc3339();
        let updated_str = task.updated.to_rfc3339();
        self.connection.execute(
            "INSERT INTO tasks (title, description, created, updated, done) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                task.title,
                task.description,
                created_str,
                updated_str,
                task.done,
            ),
        )?;
        let id = self.connection.last_insert_rowid() as i32;
        self.get(id).map(|task| task.unwrap())
    }

    fn update(&self, task: Task) -> Result<Task, StorageError> {
        let updated_str = task.updated.to_rfc3339();
        self.connection.execute(
            "UPDATE tasks SET title = ?1, description = ?2, updated = ?3, done = ?4 WHERE id = ?5",
            (
                task.title,
                task.description,
                updated_str,
                task.done,
                task.id,
            ),
        )?;
        self.get(task.id).map(|task| task.unwrap())
    }
}
