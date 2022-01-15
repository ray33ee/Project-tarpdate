use std::path::{Path, PathBuf};
use crate::toc::TOC;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use crate::builder::Builder;

const MAGIC_NUMBER: u128 = 0x169f57e6bbb98f2d139ee9a294f9cd3c;

#[derive(Debug)]
pub struct Archive {
    pub path: PathBuf,
    pub toc: TOC,
    pub toc_offset: u64,
}

impl Archive {
    pub fn create<P: AsRef<Path>>(path: P) -> Self {

        let toc = TOC::new();

        let file = OpenOptions::new().write(true).create(true).open(&path).unwrap();

        // Write the magic number to the first 16 bytes
        bincode::serialize_into(&file, &MAGIC_NUMBER).unwrap();

        //Write the offset for the TOC to the next 8 bytes
        bincode::serialize_into(&file, &24u64).unwrap();

        //Write the TOC
        bincode::serialize_into(&file, &toc).unwrap();

        Archive {
            path: PathBuf::from(path.as_ref()),
            toc,
            toc_offset: 24,
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();

        let length = file.metadata().unwrap().len();

        let magic_number: u128 = bincode::deserialize_from(&file).unwrap();

        if magic_number != MAGIC_NUMBER {
            panic!("Bad magic number");
        }

        let toc_offset: u64 = bincode::deserialize_from(&file).unwrap();

        file.seek(SeekFrom::Start(toc_offset)).unwrap();

        let toc: TOC = bincode::deserialize_from(&file).unwrap();

        if file.stream_position().unwrap() < length {
            //There is data past the end of the contents table, help! This is not an error, it should just be a warning
            panic!("Data past end of TOC")
        }

        Archive {
            path: PathBuf::from(path.as_ref()),
            toc,
            toc_offset,
        }

    }

    //Return a builder object for the current archive that adds files to the archive
    pub fn builder(& mut self) -> Builder {

        Builder::new(self)

    }

    //fn verify_magic() -> bool;
}