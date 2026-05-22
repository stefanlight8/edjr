//! Journal entry
use {
    crate::event::JournalEvent,
    chrono::{DateTime, Utc},
    serde::Deserialize,
};

// Journal entry.
//
// Represents each line of journal.
#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: JournalEvent,
}
