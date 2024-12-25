mod storage;
mod service;
mod task;

use storage::storage_sqlite::StorageSqlite;

fn main() {
    let storage = StorageSqlite::new("tasks.db").unwrap();
    println!("Hello, world!");
}
