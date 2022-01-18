use std::path::{Path, PathBuf};
use crate::archive::Archive;
use std::fs::{OpenOptions, File};
use std::io::{Seek, SeekFrom};
use crate::header::Metadata;

pub struct Builder<'a> {

    archive: & 'a  mut Archive,

    archive_file: std::fs::File,
}

impl<'a> Builder<'a> {

    pub fn new(archive: & 'a mut Archive) -> Self {
        //Truncate the file to remove the toc for overwriting
        let archive_file = OpenOptions::new().write(true).open(&archive.path).unwrap();
        archive_file.set_len(archive.toc_offset as u64).unwrap();

        //Open the file for appending
        let archive_file = OpenOptions::new().write(true).append(true).open(&archive.path).unwrap();

        Self {
            archive,
            archive_file,
        }

    }

    pub fn append<P: AsRef<Path>, N: AsRef<Path>>(&mut self, path: P, name: N) {

        //Check for naming conflicts in the toc
        if self.archive.toc._table.contains_key(path.as_ref()) {

            panic!("This file already exists {}", path.as_ref().to_str().unwrap());

        }

        //Get the position of the stream (this will be used as the file offset in the toc)
        self.archive_file.seek(SeekFrom::End(0)).unwrap();
        let position = self.archive_file.stream_position().unwrap();

        //Append the metadata
        let meta: Metadata = File::open(&path).unwrap().metadata().unwrap().into();

        bincode::serialize_into(&self.archive_file, &meta).unwrap();

        //Append the file
        {
            let mut file = OpenOptions::new().read(true).open(&path).unwrap();

            std::io::copy(& mut file, & mut self.archive_file).unwrap();
        }

        //Add the (name, file_offset) pair to the toc
        self.archive.toc._table.insert(PathBuf::from(name.as_ref()), position as u128);

    }

    pub fn finalise(mut self) {
        //Get this position of the stream (which is the size of the file at this point, without the toc)
        self.archive_file.seek(SeekFrom::End(0)).unwrap();
        let position = self.archive_file.stream_position().unwrap();

        self.archive.toc_offset = position as u128;

        //Append toc
        bincode::serialize_into(&self.archive_file, &self.archive.toc._table).unwrap();

        //Write offset to toc offset at beginning
        {
            let _f = self.archive_file;

            //Close the file
        }

        let mut file = OpenOptions::new().write(true).open(&self.archive.path).unwrap();

        file.seek(SeekFrom::Start(16)).unwrap();

        bincode::serialize_into(file, &position).unwrap();

    }

}