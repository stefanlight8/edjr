use {
    crate::event::JournalEvent,
    chrono::{DateTime, Utc},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: JournalEvent,
}
