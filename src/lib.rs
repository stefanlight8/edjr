//! # edjr
//!
//! Elite Dangerous Journal Reader
//!
//! A library for parsing Elite Dangerous's journal files.
//! edjr supports only Elite Dangerous Odyssey.
//!
//! ## Features
//! - `tokio`: provide tokio-based journal reader.
//! - `stream`: provides stream for journal reader.
//!
//! ## Examples
//! ### Read all (sync)
//! ```no_run
//! use {
//!     edjr::{journal::Journal, read::Read},
//!     std::{error::Error, fs::File, path::PathBuf},
//! };
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let journal_path =
//!         PathBuf::from("/Path/to/Journals/Journal.date.log");
//!     let mut journal = Journal::<File>::open(journal_path)?;
//!     let entries = journal.read_all()?;
//!
//!     Ok(())
//! }
//!
//! ```
//! ### Read all (async, features = [tokio])
//! ```no_run
//! use {
//!     edjr::{journal::Journal, async_read::AsyncRead},
//!     std::{error::Error, path::PathBuf},
//!     tokio::fs::File,
//! };
//!
//! #[tokio::main]
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let journal_path =
//!         PathBuf::from("/Path/to/Journals/Journal.date.log");
//!     let mut journal = Journal::<File>::open(journal_path).await?;
//!     let entries = journal.read_all().await?;
//!
//!     Ok(())
//! }
//!
//! ```
//! View more examples in [`examples/`](./examples)
pub mod async_read;
pub mod elite;
pub mod entry;
pub mod error;
pub mod event;
pub mod events;
pub mod journal;
pub mod read;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod tokio;

pub use {
    async_read::AsyncRead, entry::JournalEntry, event::JournalEvent, journal::Journal, read::Read,
};
