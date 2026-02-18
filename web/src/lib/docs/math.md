# Math

Additions and Subtractions cant be easily used with - and +, while more advanced stuff requires the math package for now.

```rust
let a = 5;
let b = -5;
let c = a - b;

print(a);
print(b);
print(c);

// Outputs
//
// 5
// -5
// 10
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
```

## Joining Strings

You can use '+' to join strings, like this:
```rust
let a = "Hello,";

print(a + " World!");
```

This should output "Hello, World!"