use {
    crate::event::JournalEvent,
    chrono::{DateTime, Utc},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    timestamp: DateTime<Utc>,
    #[serde(flatten)]
    event: JournalEvent,
}
