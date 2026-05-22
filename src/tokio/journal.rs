//! Journal
use {
    crate::{Journal, error::JournalError},
    std::path::Path,
    tokio::fs::File,
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
}
