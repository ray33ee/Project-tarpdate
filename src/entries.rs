use crate::archive::Archive;
use std::collections::hash_map::{Iter};
use crate::entry::Entry;
use crate::safepath::SafePathBuf;

///An iterator over all the active files in an archive
pub struct Entries<'a> {
    archive: & 'a Archive,
    iterator: Iter<'a, SafePathBuf, u128>,

}

impl<'a> Entries<'a> {
    pub (in crate) fn new(archive: & 'a Archive) -> Self {
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

