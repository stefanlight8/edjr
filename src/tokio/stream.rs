use {
    crate::{Journal, JournalEntry, error::JournalReadError},
    async_stream::stream,
    futures_lite::Stream,
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
};

impl Journal<File> {
    pub fn stream(self) -> impl Stream<Item = Result<JournalEntry, JournalReadError>> {
        stream! {
            let mut reader = BufReader::new(self.file);
            let mut buffer = String::new();

            loop {
                buffer.clear();
                let line = reader.read_line(&mut buffer).await.map_err(JournalReadError::ReadError)?;

                if line == 0 {
                    break;
                }

                yield serde_json::from_str::<JournalEntry>(&buffer)
                    .map_err(JournalReadError::ParsingError)
            }
        }
    }
}
