use std::path::{Path, PathBuf};
use crate::archive::Archive;
use std::fs::{OpenOptions, File};
use std::io::{Seek, SeekFrom};
use crate::header::Metadata;
use crate::safepath::SafePathBuf;
use std::convert::TryFrom;
use crate::error::{Result, Error, ErrorKind};

///A specialised object used to append files to archives
///
/// Builders are not explicitly created, but returned by [`Archive::builder`].
pub struct Builder<'a> {

    archive: & 'a  mut Archive,

    archive_file: std::fs::File,
}

impl<'a> Builder<'a> {

    pub(in crate) fn new(archive: & 'a mut Archive) -> Result<Self> {
        //Truncate the file to remove the toc for overwriting
        let archive_file = OpenOptions::new().write(true).open(archive.path())?;
        archive_file.set_len(archive.toc_offset() as u64)?;

        //Open the file for appending
        let archive_file = OpenOptions::new().write(true).append(true).open(archive.path())?;

        Ok(Self {
            archive,
            archive_file,
        })

    }

    ///Add a new file at `path` to the archive. The path stored in the archive itself is specified by `name`
    pub fn append<P: AsRef<Path>, N: AsRef<Path>>(&mut self, path: P, name: N) -> Result<()> {

        //Check for naming conflicts in the toc
        if self.archive.table()._table.contains_key(path.as_ref()) {

            return Err(Error::new(ErrorKind::PathConflict(PathBuf::from(path.as_ref())), format!("Could not append file to TOC with the chosen path ({}), as path already exists in TOC", path.as_ref().to_str().unwrap())));

        }

        //Get the position of the stream (this will be used as the file offset in the toc)
        self.archive_file.seek(SeekFrom::End(0))?;
        let position = self.archive_file.stream_position()?;

        //Append the metadata
        let meta: Metadata = File::open(&path)?.metadata()?.into();

        bincode::serialize_into(&self.archive_file, &meta)?;

        //Append the file
        {
            let mut file = OpenOptions::new().read(true).open(&path)?;

            std::io::copy(& mut file, & mut self.archive_file)?;
        }

        //Add the (name, file_offset) pair to the toc
        self.archive.toc._table.insert(SafePathBuf::try_from(name.as_ref())?, position as u128);

        Ok(())

    }

    ///Must be called when files have been appended to replace the temporarily removed toc.
    ///
    /// Because the toc is temporarily removed during appending, this function should be called as soon as possible to preserve the integrity of the archive.
    /// Any unnecessary code executed before this function is called may cause a panic and result in the loss of the toc for the archive.
    pub fn finalise(mut self) -> Result<()> {
        //Get this position of the stream (which is the size of the file at this point, without the toc)
        self.archive_file.seek(SeekFrom::End(0))?;
        let position = self.archive_file.stream_position()?;

        self.archive.toc_offset = position as u128;

        //Append toc
        bincode::serialize_into(&self.archive_file, &self.archive.toc._table)?;

        //Write offset to toc offset at beginning
        {
            let _f = self.archive_file;

            //Close the file
        }

        let mut file = OpenOptions::new().write(true).open(self.archive.path())?;

        file.seek(SeekFrom::Start(16))?;

        bincode::serialize_into(file, &position)?;

        Ok(())

    }

}