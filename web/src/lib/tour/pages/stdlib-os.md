## OS
The OS package does not work in browsers, but here is a demonstration of what it has

[CODE]
import "std/os";

// executes a command and returns an object with:
// stdout, stderr, status_code and success
print(os.exec("whoami"));

// prints the current process id
print(os.pid());

if os.name == "linux" or os.name == "macos" {
    // prints the current user id, only work on unix-like operating systems
    print(os.uid());

    // prints the current group id, only work on unix-like operating systems
    print(os.gid());
}

// gets an env var
print(os.getenv("path"));

// sets an env var
os.setenv("test", "modu");

// unset an env var
os.unsetenv("test");

// print cmdline args
print(os.args());

// print os name
// windows, linux, macos or unknown
print(os.name);