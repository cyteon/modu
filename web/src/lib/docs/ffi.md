# Foreign Function Interface (FFI)

⚠️ FFI functions can only take strings, integers, floats, booleans and null as arguments or return values

## Library functions
```txt
ffi.load(path)     - load a ffi library
ffi.define(
    lib, 
    function_name, 
    arg_types, 
    return_type
)                  - define a function from the library, so it can be called from modu
ffi.unload(lib)    - unload a library
```

## Using FFI

```rust
import "std/ffi" as ffi;

// Note that .so is the shared library extension on Linux
// On windows it would be .dll, and on MacOS it would be .dylib
// In actal code you would have to differentiate using os.name 
// (returns windows/linux/macos/unknown)
// For info on the OS package see the "OS Lib" page
let lib = ffi.load("./libffi_test.so");
ffi.define(lib, "hello_world", [], null);
lib.hello_world();

// Output:
//
// Hello, World
```

This is the **hello_world** function, written as a rust lib:
```rust
#[unsafe(no_mangle)]
pub extern "C" fn hello_world() {
    println!("Hello, World!");
}
```

Note: i am using rust cause i prefer that, you can write the libraries in any programming that exports to C-Style libraries. \
Such as:
- C (of course you can use C)
- Go (using CGO)
- Python (using ctypes)

## Arguments
To use arguments call the function like any other function: `ffi.function_name(arg1, arg2, arg3, ...)`

Here is an example:
```rust
import "std/ffi" as ffi;

let lib = ffi.load("./libffi_test.so");
ffi.define(lib, "add", ["i64", "i64"], "i64");
print(lib.add(2, 5));

// Output:
//
// 7
```

Here is the code for the library:
```rust
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    a + b
}
```