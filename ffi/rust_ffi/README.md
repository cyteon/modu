# modu_ffi
Makes implementing ffi functions for modu way easier

### Example:
In rust:
```rust
use modu_ffi::*;

#[unsafe(no_mangle)]
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

In modu:
```rust
import "ffi" as ffi;

let lib = ffi.load("./library_path.so"); // or .dll or .dylib
print(lib.add(1, 2));
```