//! Journal entry
use {
    crate::event::JournalEvent,
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
};

// Journal entry.
//
// Represents each line of journal.
#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: JournalEvent,
}
