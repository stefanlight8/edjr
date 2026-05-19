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
    pub async fn stream(
        &mut self,
    ) -> Result<impl Stream<Item = Result<JournalEntry, JournalReadError>>, JournalReadError> {
        let mut reader = BufReader::new(
            self.file
                .try_clone()
                .await
                .map_err(JournalReadError::ReadError)?,
        );
        let mut buffer = String::new();

        Ok(stream! {
            loop {
                buffer.clear();
                reader.read_line(&mut buffer).await.map_err(JournalReadError::ReadError)?;

                yield serde_json::from_str::<JournalEntry>(&buffer)
                    .map_err(JournalReadError::ParsingError)
            }
        })
    }
}
