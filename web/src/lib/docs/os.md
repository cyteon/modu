# OS Library

The built-in library to interact with the OS.

```txt
os.exec(string)       - executes a command in the terminal and returns an object with the following properties:
    stdout  - the output of the command
    stderr  - the error output of the command, if any
    code    - the exit code of the command
    success - true if the command exited with code 0, false otherwise
os.pid()              - the current process id
os.uid()              - the current user id, only works on unix-like operating systems
os.gid()              - the current group id, only works on unix-like operating systems
os.getenv(var)        - gets the value of an environment variable
os.setenv(var, value) - sets the value of an environment variable
os.unsetenv(var)      - unsets an environment variable
os.name               - the name of the operating system, can be "windows", "linux", "macos" or "unknown"
```

## Example

```rust
// UNIX-like OS only

import "os" as os;

if os.name == "windows" {
    print("this example dosent run on windows :C\nbut if u wanted to know, the pid was ", os.pid());
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