use chrono::Utc;

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created: chrono::DateTime<Utc>,
    pub updated: Option<chrono::DateTime<Utc>>,
    pub done: bool,
}

impl Task {
    pub fn new(title: String, description: String) -> Task {
        Task {
            id: 0,
            title,
            description,
            created: Utc::now(),
            updated: None,
            done: false,
        }
    }
}
