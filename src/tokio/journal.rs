//! Journal
use {
    crate::{
        Journal, JournalEntry,
        error::{JournalError, JournalReadError},
    },
    std::path::Path,
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
};

/// Journal implementation for [tokio::fs::File]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl Journal<File> {
    /// Opens a journal from a file path.
    ///
    /// # Examples
    /// ```no_run
    /// use {edjr::Journal, std::error::Error, tokio::fs::File};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let journal = Journal::<File>::open("/Path/to/my/journals/Journal.date.log").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// [JournalError::OpenError] if the file cannot be opened.
    pub async fn open(path: impl AsRef<Path>) -> Result<Journal<File>, JournalError> {
        let file = File::open(path)
            .await
            .map_err(|err| JournalError::OpenError(err))?;

        Ok(Self { file })
    }

    /// Read all entries from journal.
    ///
    /// # Example
    /// ```no_run
    /// use {std::error::Error, edjr::Journal, tokio::fs::File};
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
    pub async fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError> {
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
