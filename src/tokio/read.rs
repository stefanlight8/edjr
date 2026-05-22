use {
    crate::{Journal, async_read::AsyncRead, entry::JournalEntry, error::JournalReadError},
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
};

#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl AsyncRead for Journal<File> {
    /// Read all entries from journal.
    ///
    /// # Example
    /// ```no_run
    /// use {std::error::Error, edjr::{Journal, AsyncRead}, tokio::fs::File};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let journal = Journal::<File>::open("/Path/to/my/journals/Journal.date.log").await?;
    ///     let entries = journal.read_all().await?;
    ///
    ///     println!("{:?}", entries);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// [JournalReadError::ReadError] cause by failed reading line wikth buffered reader
    /// [JournalReadError::ParsingError] cause by failed parsing line
    async fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError> {
        let mut reader = BufReader::new(&mut self.file);
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
