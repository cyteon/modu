## FFI
The FFI package does not work in browsers, but here is a demonstration of what it has.

The FFI package lets you load shared libraries in modu, and call functions from them.
The support is currenetly pretty rudementary and only supports the following types:
i64, f64, i32, f32, bool, string, void

[CODE]
import "std/ffi";

// loads the library libffi_test.(so/dylib/dll) based on operating system
let lib = ffi.load("libffi_test." + ffi.suffix);

// defines the function add(i64, i64) -> i64
// this depends on the function existing in the library
ffi.define(lib, "add", ["i64", "i64"], "i64");

// this will run the function from the library
print(ffi.add(1, 2));

// unload the library when you are done with it
ffi.unload(lib);