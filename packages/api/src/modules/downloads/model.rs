use crate::config::DATE_FORMAT;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct Download {
    pub id: i32,
    pub queue_id: i32,
    pub source_id: i32,
    pub title: String,
    pub subtitle: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

impl Download {
    pub fn attach(self) -> DownloadJson {
        DownloadJson {
            id: self.id,
            queue_id: self.queue_id,
            source_id: self.source_id,
            title: self.title,
            subtitle: self.subtitle,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
            finished_at: self.finished_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadJson {
    pub id: i32,
    pub queue_id: i32,
    pub source_id: i32,
    pub title: String,
    pub subtitle: String,
    pub created_at: String,
    pub updated_at: String,
    pub finished_at: String,
}
