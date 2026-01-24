# File I/O
> Disabled on the server :P

Start of with importing the File pacakge with either of these two methods:
```rust
import "file" as file;
import "file" as *;
```

### Reading files

You can read files with **read(path)** function:
```rust
let content = file.read("file.txt"); // or read() if imported with *
print(content);
```

### Writing files
You can write files using **write(path, content)** or **write_append(path, content)**
```rust
file.write("file.txt", "hello");
print(file.read("file.txt"));

// Outputs:
// hello

file.write_append("file.txt", " world");
print(file.read("file.txt"));

// Outputs:
// hello world
```