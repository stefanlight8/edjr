use {
    crate::error::JournalError,
    std::{fs::File, path::Path},
};

pub struct Journal<F> {
    pub(crate) file: F,
}

impl Journal<File> {
    pub fn open(path: impl AsRef<Path>) -> Result<Journal<File>, JournalError> {
        let file = File::open(path).map_err(|err| JournalError::OpenError(err))?;

        Ok(Self { file })
    }
}
