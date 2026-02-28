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

```txt
math.PI           - 3.14...
math.E            - 2.71...
math.abs(number)  - absolute value
math.ceil(float)  - rounds up
math.floor(float) - rounds down
math.round(float) - rounds to nearest integer
math.sqrt(number) - square root
math.rand()       - random number between 0 and 1
```

## Strings

You can use `+` to join strings, like this:
```rust
let a = "Hello,";
print(a + " World!");
```

And you can use `*` to repeat strings, like this:
```rust
let a = "Hello! ";
print(a * 3);
```