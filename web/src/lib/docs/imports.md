# Imports

You can import files and installed packages with `import "path/name"`, and standard packages with `import "std/name"`. \
You can also optionally add `as alias` to change what its imported as.

```rust
// yapper.modu

let abc = 123;

fn yap(msg) {
    print(msg);
}

```

<span class="my-5" ></span>

```rust
// main.modu

import "yapper.modu" as yapper;

yapper.yap("Hello, World!");
print(yapper.abc);
```

This should output:
```txt
Hello, World!
123
```

You can also import with an * to directly import all variables and functions into the context.
```rust
import "yapper.modu" as *;

yap("Hello, World!");
print(abc);
```

## Internal packages

Internal and installed packages are imported without **.modu** like:
```rust
import "std/math" as math;
import "std/file" as file;

let a = math.abs(-5);
let b = file.open("input.modu");
```
or alternatively
```rust
import "std/math"; // it will import without the std/ prefix
import "std/file";

let a = math.abs(-5);
let b = file.open("input.modu");
```
or even
```rust
import "std/math" as *;
import "std/file" as *;

let a = abs(-5);
let b = open("input.modu");
```