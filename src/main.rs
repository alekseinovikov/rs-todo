mod gui;
mod service;
mod storage;
mod task;

use crate::gui::Gui;
use service::service_impl::ServiceImpl;
use storage::storage_sqlite::StorageSqlite;

const DB_FILE: &str = "tasks.db";

fn main() {
    let storage = StorageSqlite::new(DB_FILE).unwrap();
    let service = ServiceImpl::new(Box::new(storage)).unwrap();
    Gui::new(Box::new(service)).run();
}
