# Math

```rust
let a = 5;
let b = -5;
let c = a - b;
let d = 5 * 5;
let e = 10 / 2;
let f = 3 ** 2;
let g = 10 % 3;

print(a);
print(b);
print(c);

// Outputs
//
// 5
// -5
// 10
// 25
// 5
// 9
// 1
```

## Math Package

You can import the package with
```rust
import "math" as math;

math.div(1,2); // can be used like this
```
or
```rust
import "math" as *; // can be accessed without any prefix

div(1,2); // is not a property now
```

You can do the following with the math package, it's not much but i will add more asap
```rust
print("mul(2, 3)    = 6   = ", math.mul(2, 3));
print("div(7, 2)    = 3.5 = ", math.div(7, 2));
print("abs(-5)      = 5   = ", math.abs(-5));
print("pow(3, 2)    = 9   = ", math.pow(3, 2));
```

## Joining Strings

You can use '+' to join strings, like this:
```rust
let a = "Hello,";

print(a + " World!");
```

This should output "Hello, World!"