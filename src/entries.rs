use crate::archive::Archive;
use std::collections::hash_map::{Keys, Iter};
use std::path::PathBuf;
use crate::entry::Entry;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};

pub struct Entries<'a> {
    archive: & 'a Archive,
    iterator: Iter<'a, PathBuf, u128>,

}

impl<'a> Entries<'a> {
    pub fn new(archive: & 'a Archive) -> Self {
        Entries {
            archive,
            iterator: archive.toc._table.iter(),
        }
    }

}

impl<'a> Iterator for Entries<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (path, offset) = self.iterator.next()?;

        Some(Entry::new(path.as_path(), *offset, self.archive))
    }
}

