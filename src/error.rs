
pub type Result<T> =  std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    IO(std::io::Error),
    Bincode(bincode::Error),
    BadMagicNumber(u128),
    TocEntryNotFound,
}

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
