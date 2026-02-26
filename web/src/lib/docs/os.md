# OS Library

The built-in library to interact with the OS.

Currently has the following functions:
- `exec(command)` - Runs a command

And the following variables:
- `name` - Returns the OS name: windows/linux/macos/unkown

Example:

```rust
// UNIX-like OS only

import "os" as os;

if os.name == "windows" {
    print("this example dosent run on windows :C");
    exit();
}

print("I am ", os.exec("whoami"), " and I use ", os.name);

os.exec("echo 'Hello, World!' > tmp.txt");
print(os.exec("cat tmp.txt"));
os.exec("rm tmp.txt");

// Expected Output
//
// I am <username> and I use <operating system>
// Hello, World!
```