use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateEntryBody {
    pub title: String,
    pub date: i64
}

#[derive(Deserialize)]
pub struct UpdateEntryBody {
    pub title: String
}