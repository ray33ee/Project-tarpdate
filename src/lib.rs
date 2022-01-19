
///Serialisable file metadata structs
pub mod header;

///Main object used to create, access and modify tarpdata archives
pub mod archive;

///Object used to add files to archive
pub mod builder;

///Iterator over files in archive
pub mod entries;

///Object representing an archived file
pub mod entry;

///An archive-safe wrapper around Path
pub mod safepath;

///Objects used in tarpdate-specific errors
pub mod error;

mod toc;

#[cfg(test)]
mod tests {
    use crate::archive::Archive;
    use std::path::{PathBuf};
    use std::io::{Read, Seek, SeekFrom};
    use crate::safepath::SafePathBuf;
    use std::convert::TryFrom;

    #[test]
    fn archive_test() {
        let path = "./test/test.t";
        let mut archive = Archive::create(path).unwrap();

        let mut builder = archive.builder().unwrap();

        builder.append("./test/a", "./a").unwrap();
        builder.append("./test/b", "./b").unwrap();



        builder.finalise().unwrap();

        for mut entry in archive.iter() {
            println!("path: {:?}, header: {:?}", entry.path(), entry.header());

            let mut v = String::new();

            entry.seek(SeekFrom::Start(3)).unwrap();

            entry.read_to_string(& mut v).unwrap();

            println!("Data: {}", v);
        }

        println!("{:?}", archive.get(PathBuf::from("./a").as_path()));

        archive.remove("./b").unwrap();

        for mut entry in archive.iter() {
            println!("path: {:?}, header: {:?}", entry.path(), entry.header());

            let mut v = String::new();

            entry.read_to_string(& mut v).unwrap();

            println!("Data: {}", v);
        }

        println!("Walk: {:?}", archive.walk());

    }

    #[test]
    fn safe_path() {

        SafePathBuf::try_from(PathBuf::from("things/stuff\\f")).unwrap();
        assert!(SafePathBuf::try_from(PathBuf::from("/dfd/../sdf")).is_err());

    }

    #[test]
    fn unsafe_archive() {
        assert!(Archive::open("./test/unsafe archive.t").is_err());
    }
}
