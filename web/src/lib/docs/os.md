# OS Library

The built-in library to interact with the OS.

Currently has the following functions:
- `exec(command)` - Run a command, returns `{ stderr: string, stdout: string, status_code: i64, success: bool }`
- `pid()` - Returns the current process ID
- `uid()` - Returns the user ID of the current user (UNIX-like OS only)
- `gid()` - Returns the group ID of the current user (UNIX-like OS only)
- `getenv(key)` - Returns the value of an environment variable
- `setenv(key, value)` - Sets the value of an environment variable
- `unsetenv(key)` - Unsets an environment variable

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

print("I am ", os.exec("whoami").stdout.trim(), " and I use ", os.name);

os.exec("echo 'Hello, World!' > tmp.txt");
print(os.exec("cat tmp.txt").stdout.trim());
os.exec("rm tmp.txt");

// Expected Output
//
// I am <username> and I use <operating system>
// Hello, World!
```