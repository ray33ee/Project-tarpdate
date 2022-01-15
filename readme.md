# Project Tarpdata

This project aims to update the archaic [tar](https://en.wikipedia.org/wiki/Tar_(computing)) archiving tool into a modern and fast archiving tool. 

## Why?

The tar archive has many shortcomings, and this project aims to address these issues while keeping the format as simple as possible. Project Tarpdate is currently a Rust library, but hopes top expand into an archiving executable.

## tar Limitations

### 512-block size

- Split into 512 blocks with padding
- Unnecessary two empty blocks at end

(Solved: We don't use blocks)

### Sequential reading

- No TOC means sequential reads

(Solved: We create a TOC at the end of each archive. If this fails or is missing, we can rebuild it by sequentially reading it)

### File and path size limits

- Paths are limited to a certain size , and file sizes are limited too (68gb)

(Solved: file sizes are stored as u64, and paths are variable length, up to 2^32 characters)

### Large headers

- Entries are stored in octal
- Does not respect UNIX permissions

(Solved: headers store the data in binary)

### Duplicates

- Duplicate files with the same path are allowed under tar (and each one replaces the previous) 

(Solved: When we add a new file, we search the TOC for a conflict )

### No Deletions

- Deletions from tar would be difficult as the forward data would need to be shifted into the deleted space, since the list must be continuous

(Solved: Deleting a file is as easy as removing an entry from the TOC. A function to defrag can be added too)

### Tarbomb

- Files in an extracted archive can 'explode' in a directory overwriting files of the same name, or mixing files up

