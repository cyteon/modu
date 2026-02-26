# Math

Example operations not requiring the math package:
```rust
let a = 5;
let b = -5;
let c = a - b;
let d = 5 * 5;
let e = 10 / 2;
let f = 3 ** 2;
let g = 10 % 3;
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

You can do the following with the math package currently:
```rust
print(math.PI);
print(math.E);
print(math.abs(-5));
print(math.ceil(math.PI));
print(math.floor(math.E));
print(math.round(1.6));
print(math.sqrt(16));
print(math.rand());
```

## Strings

You can use '+' to join strings, like this:
```rust
let a = "Hello,";
print(a + " World!");
```

And you can use '*' to repeat strings, like this:
```rust
let a = "Hello! ";
print(a * 3);
```