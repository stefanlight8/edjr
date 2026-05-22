//! Asyncronous reading
use crate::{entry::JournalEntry, error::JournalReadError};

pub trait AsyncRead {
    /// Read all entries from journal.
    ///
    /// Requires one of available asynchronous backends: [tokio]
    async fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError>;
}
