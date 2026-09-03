//! Journal
use {
    crate::{
        JournalEntry,
        error::{JournalError, JournalReadError},
    },
    std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    },
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

    // Read all entries from journal.
    ///
    /// # Example
    /// ```no_run
    /// use {std::{error::Error, fs::File}, edjr::Journal};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let journal = Journal::<File>::open("/Path/to/my/journals/Journal.date.log")?;
    ///     let entries = journal.read_all()?;
    ///
    ///     println!("{:?}", entries);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError> {
        let reader = BufReader::new(&self.file);

        serde_json::Deserializer::from_reader(reader)
            .into_iter::<JournalEntry>()
            .map(|entry| entry.map_err(|err| JournalReadError::ParsingError(err)))
            .collect()
    }

    /// Get iterator over journal.
    ///
    /// # Example
    /// ```no_run
    /// use {std::{error::Error, fs::File}, edjr::Journal};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let journal = Journal::<File>::open("/Path/to/my/journals/Journal.date.log")?;
    ///
    ///     for event in journal.iter() {
    ///         let event = event?;
    ///
    ///         println!("{:?}", event);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter(self) -> impl Iterator<Item = Result<JournalEntry, JournalReadError>> {
        let reader = BufReader::new(self.file);

        reader.lines().map(|line| {
            let line = line.map_err(JournalReadError::ReadError)?;

            serde_json::from_str::<JournalEntry>(&line).map_err(JournalReadError::ParsingError)
        })
    }
}
