mod header;
mod toc;
mod archive;
mod builder;



#[cfg(test)]
mod tests {
    use crate::archive::Archive;

    #[test]
    fn archive_test() {
        let path = "./test/test.t";
        let mut archive = Archive::create(path);

        let mut builder = archive.builder();

        builder.append("./test/a", "./a");
        builder.append("./test/b", "./b");

        builder.finalise();

        println!("archive: {:?}", archive);

    }
}
