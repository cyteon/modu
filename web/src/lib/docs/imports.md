# Imports

We currently only support importing other files and internal packages, no external ones yet \
When you import a file, you can access it's variables and functions using pkg.*, if you imported with "... as pkg". \
'pkg' can be any non-reserved keyword.

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

import "yapper.modu" as yapper

yapper.yap("Hello, World!");
print(yapper.abc);
```

This should output:
```
Hello, World!
123
```

You can also import with an * to remove the need of "yapper." on the start
```rust
import "yapper.modu" as *

yap("Hello, World!");
print(abc);
```

## Internal packages

Internal/Installed packages are imported without **.modu** like:
```rust
import "math" as math;
import "file" as file;

let a = math.abs(-5);
let b = file.read("input.modu");
```
or alternatively
```rust
import math;
import file;

let a = math.abs(-5);
let b = file.read("input.modu");
```
or even
```rust
import "math" as *;
import "file" as *;

let a = abs(-5);
let b = read("input.modu");
```