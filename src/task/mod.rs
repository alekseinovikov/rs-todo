use chrono::{DateTime, Utc};

pub struct ShortTask {
    pub id: i32,
    pub title: String,
    pub updated: DateTime<Utc>,
    pub done: bool,
}

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub done: bool,
}

impl Task {
    pub fn new(title: String, description: String) -> Task {
        Task {
            id: 0,
            title,
            description,
            created: Utc::now(),
            updated: Utc::now(),
            done: false,
        }
    }
}
