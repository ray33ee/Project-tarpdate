use std::fmt::{Display, Formatter};
use std::path::PathBuf;

///A specialised Result type for tarpdate
pub type Result<T> =  std::result::Result<T, Error>;

///An enum representing the reason that a toc could not be loaded.
///
/// See [`ErrorKind::TocEntryNotFound`] for more information
#[derive(Debug)]
pub enum TocEntryNotFoundReason {
    ///Stored toc offset value is past the end of file
    ///
    /// Contains the toc offset and size of file
    TocOffsetPastEOF(u128, u128),

    /// Serde returned an error when deserialising the toc
    ///
    /// Contains the bincode error
    CouldNotDeserialiseToc(bincode::Error)
}

///A list of possible tarpdate errors
#[derive(Debug)]
pub enum ErrorKind {
    ///An error from I/O operations
    IO(std::io::Error),

    ///An error from [`bincode`]
    Bincode(bincode::Error),

    ///Magic number does not match.
    ///
    /// Contains a `u128` with the magic number found
    BadMagicNumber(u128),

    ///Table of contents could not be serialised.
    ///
    /// Contains the reason that the toc could not be loaded
    TocEntryNotFound(TocEntryNotFoundReason),

    ///Table contains or is trying to write an unsafe path
    ///
    /// Contains the offending path
    UnsafePath(PathBuf),

    ///Could not append file to TOC with the chosen path, as path already exists in TOC
    ///
    /// Contains the offending path
    PathConflict(PathBuf),
}

///An error type encapsulating possible errors from tarpdata operations
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: String,
}

impl Error {
    pub fn new(kind: ErrorKind, error: String) -> Self {
        Self {
            kind,
            error,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?} - {}", self.kind, self.error)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        let error = e.to_string();
        Self {
            kind: ErrorKind::IO(e),
            error,
        }
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        let error = e.to_string();

        Self {
            kind: ErrorKind::Bincode(e),
            error,
        }
    }
}
