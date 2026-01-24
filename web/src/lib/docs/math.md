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

You can do the following with the math package:
```rust
math.div(1, 3)    // 0.3333333333333333
math.mul(2, 3)    // 5
math.abs(-5)      // 5
math.pow(2, 3)    // 8
math.sqrt(9)      // 3
math.ceil(1.5)    // 2
math.floor(1.5)   // 1
math.random()     // 0.5526424381102485 <random float>
math.random_int() // -1130539238697420584 <random int>
math.PI           // 3.141592653589793
```

## Joining Strings

You can use '+' to join strings, like this:
```rust
let a = "Hello,";

print(a + " World!");
```

This should output "Hello, World!"