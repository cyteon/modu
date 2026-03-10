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

## Number Methods

```txt
<int>.min(x)       - minimum of the integer and x
<int>.max(x)       - maximum of the integer and x
<int>.abs()        - absolute value of the integer
<int>.sqrt()       - square root of the integer

<float>.min(x)     - minimum of the float and x
<float>.max(x)     - maximum of the float and x
<float>.abs()      - absolute value of the float
<float>.sqrt()     - square root of the float
<float>.round()    - round the float to the nearest integer
<float>.ceil()     - round the float up to the nearest integer
<float>.floor()    - round the float down to the nearest integer
```

## Math Package

```txt
math.PI           - 3.14...
math.E            - 2.71...
math.rand()       - random float between 0 and 1
math.randi()      - random integer
math.sin(x)       - sine of x (x in radians)
math.cos(x)       - cosine of x (x in radians)
math.tan(x)       - tangent of x (x in radians)
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