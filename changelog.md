# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Add correct error handling with custom error enum and structs. Whole nine yards...
- Respect UNIX permissions too
- Detect if a TOC is missing (toc offset is end of file) or corrupt (cannot serialise toc) and then fix it
- Add defrag function to remove unused space created by deleted files
- DO NOT allow parent paths '..' of any kind. Check when extracting and archiving. 
  - Maybe create a special 'Path' and 'PathBuf' that detect parent paths when initialised 
- Figure out a way to write the metadata and permissions to extracted files
- Make sure that an archive cannot be corrupted if the program fails while the toc is out of the file.
- Create a more compact and smarter serialisation 

### Unfinished Ideas
- How to fix the tarbomb issue?

## [0.1.0] - 2022-01-15
### Added
- Initial commit