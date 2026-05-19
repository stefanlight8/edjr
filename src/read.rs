use {
    crate::{entry::JournalEntry, error::JournalReadError, journal::Journal},
    std::{fs::File, io::BufReader},
};

pub trait Read {
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
