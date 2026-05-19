use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::{Journal, async_read::AsyncRead, entry::JournalEntry, error::JournalReadError};

impl AsyncRead for Journal<File> {
    async fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError> {
        let mut reader = BufReader::new(
            self.file
                .try_clone()
                .await
                .map_err(JournalReadError::ReadError)?,
        );
        let mut buffer = String::new();
        let mut entries = Vec::new();

        loop {
            buffer.clear();
            reader
                .read_line(&mut buffer)
                .await
                .map_err(JournalReadError::ReadError)?;

            if buffer.is_empty() {
                break;
            }

            entries.push(serde_json::from_str(&buffer).map_err(JournalReadError::ParsingError)?);
        }

        Ok(entries)
    }
}
