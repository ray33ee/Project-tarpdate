use std::path::Path;
use crate::header::Metadata;
use crate::archive::Archive;
use std::fs::{OpenOptions, File};
use std::io::{Seek, SeekFrom, Read, Take};
use std::io::SeekFrom::Start;

#[derive(Debug)]
pub struct Entry<'a> {
    path: & 'a Path,
    header: Metadata,
    file_offset: u128,
    reader: Take<File>,
}

impl<'a> Entry<'a> {
    pub fn new(path: & 'a Path, header_offset: u128, archive: &Archive) -> Self {

        let mut file = OpenOptions::new().read(true).open(&archive.path).unwrap();

        file.seek(SeekFrom::Start(header_offset as u64)).unwrap();

        let header: Metadata = bincode::deserialize_from(&file).unwrap();

        let file_offset = file.stream_position().unwrap() as u128;

        file.seek(SeekFrom::Start(file_offset as u64)).unwrap();

        let reader = file.take( header.len() as u64);

        Self {
            path,
            header,
            file_offset,
            reader
        }
    }

    pub fn path(&self) -> &Path {
        self.path
    }

    pub fn header(&self) -> &Metadata {
        &self.header
    }

    pub fn offset(&self) -> u128 {
        self.file_offset
    }
}

impl<'a> Read for Entry<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<'a> Seek for Entry<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {

        match pos {
            SeekFrom::Start(pos) => {
                self.reader.set_limit(self.header.len() as u64 - pos);
                let under = self.reader.get_mut();
                under.seek(Start(self.file_offset as u64 + pos))
            }
            SeekFrom::End(_) => {
                todo!()
            }
            SeekFrom::Current(_) => {
                todo!()
            }
        }
    }
}

