//! Journal reading
use {
    crate::{entry::JournalEntry, error::JournalReadError, journal::Journal},
    std::{fs::File, io::BufReader},
};

pub trait Read {
    /// Read all entries from journal.
    ///
    /// # Example
    /// ```no_run
    /// use {std::{error::Error, fs::File}, edjr::{Journal, Read}};
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
    fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError>;
}

impl Read for Journal<File> {
    fn read_all(&mut self) -> Result<Vec<JournalEntry>, JournalReadError> {
        let reader = BufReader::new(&self.file);

        serde_json::Deserializer::from_reader(reader)
            .into_iter::<JournalEntry>()
            .map(|entry| entry.map_err(|err| JournalReadError::ParsingError(err)))
            .collect()
    }
}
