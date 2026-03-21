## FS
The FS package does not work in browsers, but here is a demonstration of what it has

[CODE]
import "std/fs";

// create a directory
fs.mkdir("test");

// checks if a directory exists
print(fs.exists("test"));

// deletes a directory
fs.rmdir("test");

// opens a file in read+write
// modes are, r = read, w = write, a = append, rw = read+write
let file = fs.open("test.txt", "rw");

// writes or appends content to a file
// based on the mode it was opened with
fs.write(file, "hi");

// prints the file content
print(fs.read(file));

// print information about a file
print(fs.stat(file));

// closes a file
fs.close(file);

// deletes a file
fs.remove("test.txt");