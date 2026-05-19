use crate::{entry::JournalEntry, error::JournalReadError};

pub trait AsyncRead {
    async fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError>;
}
