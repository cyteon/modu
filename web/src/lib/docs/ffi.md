# Foreign Function Interface (FFI)

⚠️ FFI functions can only take strings, integers, floats, booleans and null as arguments

For writing ffi modules in rust, you should use [modu_ffi](https://crates.io/crates/modu_ffi) for rust. \
For C or anything else, check out the C headers: [modu_ffi.h](https://github.com/cyteon/modu/blob/1.0.0/ffi/modu_ffi.h).

Using FFI is actually really simple, just import a **.dll/.so/.dylib** file and u can run its functions with `ffi.function_name()`, here is an example:
```rust
import "ffi" as ffi;

// Note that .so is the shared library extension on Linux
// On windows it would be .dll, and on MacOS it would be .dylib
// In actal code you would have to differentiate using os.name 
// (returns windows/linux/macos/unknown)
// For info on the OS package see the "OS Lib" page
let lib = ffi.load("./libffi_test.so");
lib.hello_world();

// Output:
//
// Hello, World
```

This is the **hello_world** function, written as a rust lib:
```rust
// https://crates.io/crates/modu_ffi
use modu_ffi::*;

#[unsafe(no_mangle)]
pub extern "C" fn hello_world() -> FFIValue {
    println!("Hello, World!");
    FFIValue::null()
}
```

Note: i am using rust cause i prefer that, you can write the libraries in any programming that exports to C-Style libraries. \
Here are some examples:
- C (of course you can use C)
- Go (using CGO)
- Python (using ctypes)


## Arguments
To use arguments call the function like any other function: `ffi.function_name(arg1, arg2, arg3, ...)`

Here is an example:
```rust
import "ffi" as ffi;

let lib = ffi.load("./libffi_test.so");
print(lib.add(2, 5));

// Output:
//
// 7
```

Here is the code for the library:
```rust
// https://crates.io/crates/modu_ffi
use modu_ffi::*;

// Use no_mangle to preserve the function name
#[unsafe(no_mangle)]
// extern "C" is needed so it works (i dont have a better explanation)
pub extern "C" fn add(
    argc: std::ffi::c_int,
    argv: *const FFIValue
) -> FFIValue {
    if argc != 2 {
        panic!("add requires 2 arguments");
    }

    unsafe {
        let a = (*argv.offset(0 as isize)).value.integer;
        let b = (*argv.offset(1 as isize)).value.integer;

        FFIValue::integer(a + b)
    }
}
```