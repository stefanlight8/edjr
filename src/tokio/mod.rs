//! [tokio] based edjr's backend
//!
//! Allows to use asynchronous methods for journal such as: [edjr::AsyncRead]
//!
//! # Features
//! - stream: Allows to stream entries from journal.
pub mod journal;

#[cfg(feature = "stream")]
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
pub mod stream;
