use {
    crate::{Journal, JournalEntry, error::JournalReadError},
    async_stream::stream,
    futures_lite::Stream,
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
};

#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
// Streaming implementation for [tokio::fs::File]
impl Journal<File> {
    /// Returns a stream of journal entries.
    ///
    /// Stream will return results with [JournalEntry], or with [JournalReadError] in case of error while reading or parsing, until last line.
    ///
    /// # Example
    /// ```no_run
    /// use {std::error::Error, edjr::Journal, tokio::fs::File, futures_lite::StreamExt};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let journal = Journal::<File>::open("/Path/to/my/journals/Journal.date.log").await?;
    ///     let mut stream = journal.stream().boxed();
    ///
    ///     while let Some(entry) = stream.next().await {
    ///         match entry {
    ///             Ok(entry) => println!("{:?}", entry),
    ///             Err(err) => eprintln!("failed to read event: {}", err),
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio", feature = "stream")))]
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
