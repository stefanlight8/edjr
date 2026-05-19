use {
    crate::{Journal, error::JournalError},
    std::path::Path,
    tokio::fs::File,
};

impl Journal<File> {
    pub async fn open(path: impl AsRef<Path>) -> Result<Journal<File>, JournalError> {
        let file = File::open(path)
            .await
            .map_err(|err| JournalError::OpenError(err))?;

        Ok(Self { file })
    }
}
