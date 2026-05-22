//! Journal
use {
    crate::error::JournalError,
    std::{fs::File, path::Path},
};

/// Journal file handler.
///
/// Provides journal implementations for file.
pub struct Journal<F> {
    pub(crate) file: F,
}

/// Journal implementation for [std::fs::File]
impl Journal<File> {
    /// Opens a journal from a file path.
    ///
    /// # Examples
    /// ```no_run
    /// use {edjr::Journal, std::{fs::File, error::Error}};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let journal = Journal::<File>::open("/Path/to/my/journals/Journal.date.log")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// [JournalError::OpenError] if the file cannot be opened.
    pub fn open(path: impl AsRef<Path>) -> Result<Journal<File>, JournalError> {
        let file = File::open(path).map_err(|err| JournalError::OpenError(err))?;

        Ok(Self { file })
    }
}
