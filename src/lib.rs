mod header;
mod toc;
mod archive;
mod builder;
mod entries;
mod entry;
mod safepath;
mod error;


#[cfg(test)]
mod tests {
    use crate::archive::Archive;
    use std::path::{Path, PathBuf};
    use std::io::{Read, Seek, SeekFrom};

    #[test]
    fn archive_test() {
        let path = "./test/test.t";
        let mut archive = Archive::create(path).unwrap();

        let mut builder = archive.builder();

        builder.append("./test/a", "./a");
        builder.append("./test/b", "./b");

        builder.finalise();

        for mut entry in archive.iter() {
            println!("path: {:?}, header: {:?}, file: {}", entry.path(), entry.header(), entry.offset());

            let mut v = String::new();

            entry.seek(SeekFrom::Start(3));

            entry.read_to_string(& mut v).unwrap();

            println!("Data: {}", v);
        }

        println!("{:?}", archive.get(PathBuf::from("./a").as_path()));

        archive.remove("./b");

        for mut entry in archive.iter() {
            println!("path: {:?}, header: {:?}, file: {}", entry.path(), entry.header(), entry.offset());

            let mut v = String::new();

            entry.read_to_string(& mut v).unwrap();

            println!("Data: {}", v);
        }

        println!("Walk: {:?}", archive.walk());

    }
}
