use std::path::{Path, PathBuf};
use crate::toc::TOC;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use crate::builder::Builder;
use std::collections::HashMap;
use crate::entries::Entries;
use crate::entry::Entry;
use crate::header::Metadata;
use std::ops::Deref;
use crate::error::{Result, Error};

const MAGIC_NUMBER: u128 = 0x169f57e6bbb98f2d139ee9a294f9cd3c;

#[derive(Debug)]
pub struct Archive {
    pub path: PathBuf,
    pub toc: TOC,
    pub toc_offset: u128,
}

impl Archive {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {

        let toc = TOC::new();

        let archive_file = OpenOptions::new().write(true).create(true).open(&path)?;

        let toc_offset = 32u128; //The position of the TOC in an empty archive, which is 32 bytes in (16 bytes of magic number, 16 bytes for the stored offset itself)

        // Write the magic number to the first 16 bytes
        bincode::serialize_into(&archive_file, &MAGIC_NUMBER)?;

        //Write the offset for the TOC to the next 16 bytes
        bincode::serialize_into(&archive_file, &toc_offset)?;

        //Write the TOC
        bincode::serialize_into(&archive_file, &toc)?;

        Ok(Archive {
            path: PathBuf::from(path.as_ref()),
            toc,
            toc_offset,
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {

        let mut archive_file = OpenOptions::new().read(true).open(path.as_ref())?;

        let length = archive_file.metadata()?.len();

        let (toc, toc_offset) = Self::fetch_toc(path.as_ref())?;

        if archive_file.stream_position()? < length {
            //There is data past the end of the contents table, help! This is not an error, it should just be a warning
            panic!("Data past end of TOC")
        }

        Ok(Archive {
            path: PathBuf::from(path.as_ref()),
            toc,
            toc_offset,
        })

    }

    //Return a builder object for the current archive that adds files to the archive
    pub fn builder(& mut self) -> Builder {

        Builder::new(self)

    }

    pub fn table(&self) -> &HashMap<PathBuf, u128> {
        &self.toc._table
    }

    pub fn iter(&self) -> Entries {
        Entries::new(self)
    }

    pub fn get<'b>(&self, path: & 'b Path) -> Entry<'b> {

        let header_offset = self.toc._table.get(path).unwrap();

        Entry::new(path, *header_offset, self)
    }

    fn fetch_toc<P: AsRef<Path>>(path: P) -> Result<(TOC, u128)> {
        let mut archive_file = OpenOptions::new().read(true).open(&path)?;

        let magic_number: u128 = bincode::deserialize_from(&archive_file)?;

        if magic_number != MAGIC_NUMBER {
            panic!("Bad magic number");
        }

        let toc_offset: u128 = bincode::deserialize_from(&archive_file)?;

        archive_file.seek(SeekFrom::Start(toc_offset as u64))?;

        Ok((bincode::deserialize_from(&archive_file).unwrap(), toc_offset))
    }

    pub fn remove<P: AsRef<Path>>(& mut self, path: P) -> Result<()> {

        self.toc._table.remove(path.as_ref()).unwrap();

        {
            let mut archive_file = OpenOptions::new().write(true).open(&self.path)?;

            archive_file.set_len(self.toc_offset as u64)?;
        }

        let archive_file = OpenOptions::new().write(true).append(true).open(&self.path)?;

        bincode::serialize_into(&archive_file, &self.toc)?;

        Ok(())
    }

    pub fn defrag() {

    }

    //Walk the archive the old fashioned way
    pub fn walk(&self) -> Result<Vec<u128>> {
        let mut offsets = Vec::new();

        let mut archive_file = OpenOptions::new().read(true).open(&self.path)?;

        let archive_length = archive_file.metadata()?.len();

        archive_file.seek(SeekFrom::Start(32));

        //Iterate over each header until we cannot serialise anymore, or we serialise a file length outside the archive
        loop {

            let header_offset = archive_file.stream_position()?;


            let header: bincode::Result<Metadata> = bincode::deserialize_from(&archive_file);

            let header = match header {
                Ok(h) => {
                    h
                }
                Err(e) => {
                    println!("e: {:?}", e.as_ref());

                    if let bincode::ErrorKind::SizeLimit = e.as_ref() {
                        break;
                    }

                    if let bincode::ErrorKind::Io(d) = e.as_ref() {
                        if let std::io::ErrorKind::UnexpectedEof = d.kind() {
                            break;
                        }
                    }

                    //If we get here, then there was an unrecoverable serde error
                    return Err(Error::from(e));
                }
            };

            archive_file.seek(SeekFrom::Current(header.len() as i64))?;

            if archive_file.stream_position()? > archive_length {
                break;
            }

            offsets.push(header_offset as u128);

        }

        Ok(offsets)
    }

    //fn verify_magic() -> bool;
}