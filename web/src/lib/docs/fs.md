# Filesystem

The fs package is imported with `import "fs";` and has the following functions:
```txt
fs.open(path)        - opens a file and returns a file object
fs.read(file)        - reads the contents of a file object and returns it as a string
fs.write(file, data) - writes the given data to a file object
fs.stat(file)        - returns an object with information about the file
fs.exists(path)      - returns true if the file exists, false otherwise
fs.mkdir(path)       - creates a new directory at the given path
```

## Files
In order to read or write to an file, you first have to open it:
```rust
import "std/fs";
let file = fs.open("file.txt", "r"); // "r" for read, "w" for write, "a" for append, "rw" for read and write
```