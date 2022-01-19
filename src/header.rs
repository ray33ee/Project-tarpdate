

use serde::{Serialize, Deserialize};
use std::time::SystemTime;

///Represents the file type
#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    ///Metadata is for a directory. (see [`std::fs::Metadata::is_dir`])
    Dir,
    ///Metadata is for a regular file. (see [`std::fs::Metadata::is_file`])
    File,
    ///Metadata is for a symbolic link. (see [`std::fs::Metadata::is_symlink`])
    SystemLink,
}

///A serialisable representation of file metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    file_type: FileType,
    size: u128,
    permissions: u8,
    modified: Option<SystemTime>,
    accessed: Option<SystemTime>,
    created: Option<SystemTime>,
}

impl From<std::fs::FileType> for FileType {
    fn from(ft: std::fs::FileType) -> Self {
        if ft.is_dir() {
            FileType::Dir
        } else if ft.is_file() {
            FileType::File
        } else {
            FileType::SystemLink
        }
    }
}

impl From<std::fs::Metadata> for Metadata {
    fn from(data: std::fs::Metadata) -> Self {
        Metadata {
            file_type: data.file_type().into(),
            size: data.len() as u128,
            permissions: if data.permissions().readonly() { 1 } else { 0 },
            modified: data.modified().ok(),
            accessed: data.accessed().ok(),
            created: data.created().ok()
        }
    }
}

impl Metadata {
    ///Length of the data in the file
    pub fn len(&self) -> u128 { self.size }
}
