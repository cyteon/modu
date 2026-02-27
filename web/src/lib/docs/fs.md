# Filesystem

### Opening files
In order to read or write to an file, you first have to open i
```rust
import "fs";
let f = fs.open("file.txt");
```

### Reading files

You can read files with `f.read()` function:
```rust
let content = f.read();
print(content);
```

### Writing files
You can write files using `f.write(content)` or `f.append(content)`
```rust
f.write("hello");
print(f.read());

// Outputs:
// hello

f.append(" world");
print(f.read());

// Outputs:
// hello world
```