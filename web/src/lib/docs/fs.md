# Filesystem

The fs package is imported with `import "fs";` and has the following functions:
```txt
fs.open(path)   - opens a file and returns a file object
fs.exists(path) - returns true if the file exists, false otherwise
fs.mkdir(path)  - creates a new directory at the given path
fs.rmdir(path)  - removes the directory at the given path
fs.remove(path) - removes the file at the given path
```

## Files
In order to read or write to an file, you first have to open it:
```rust
import "fs";
let file = fs.open("file.txt");
```

You can then do the following:
```txt
file.read()         - returns the content of the file as a string
file.write(string)  - overwrites the file with the given string
file.append(string) - appends to the end of the file instead of overwriting it
file.stat()         - returns an object with information about the file, such as size and creation date
file.close()        - closes the file, you should always do this when you are done with a file
```