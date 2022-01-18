# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Respect UNIX permissions too
- Detect if a TOC is missing (toc offset is end of file) or corrupt (cannot serialise toc) and then fix it
- Add defrag function to remove unused space created by deleted files
- DO NOT allow parent paths '..' of any kind. Check when extracting and archiving. 
  - Maybe create a special 'Path' and 'PathBuf' that detect parent paths when initialised 
- Figure out a way to write the metadata and permissions to extracted files
- Make sure that an archive cannot be corrupted if the program fails while the toc is out of the file.
- Create a more compact and smarter serialisation 
- Create docs
- Finish off the `Seek::seek` implementation for `Entry`

### Unfinished Ideas
- How to fix the tarbomb issue?

## [0.1.1] - 2022-01-15
### Fixed
- File metadata bug fixed (we now use the metadata from the archive files, not the actual archive itself)

### Added
- `Entry` that represents a file in an archive
- `Entries` which is an iterator over `Entry`s in an archive
- `Read` and `Seek` added to `Entry` for reading the file data
- `Archive::remove` to remove a file from the toc
- `SafePath` skeleton
- `Error`, `ErrorKind` and `Result` for error handling
- `Archive::walk` to walk the files in the archive without the `TOC`

### Changed
- `u64` file sizes and offsets changed to `u128`

## [0.1.0] - 2022-01-15
### Added
- Initial commit