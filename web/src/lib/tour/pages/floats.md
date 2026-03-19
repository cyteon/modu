## Floats
In modu, floats are f64 numbers, and they come with the following methods:
- `<float>.max(b)`
- `<float>.min(b)`
- `<float>.abs()`
- `<float>.sqrt()`
- `<float>.round()`
- `<float>.floor()`
- `<float>.ceil()`

[CODE]
let f = 1.5;
print(f);

// prints 1.4 if its smaller than f, otherwise it prints f
print(f.min(1.4));

// prints 1.6 if its bigger than f, otherwise it prints f
print(f.max(1.6));

// prints the absolute value of f
print(f.abs());

// prints the sqrt of f
print(f.sqrt());

// prints rounded f
print(f.round());

// prints floored f
print(f.floor());

// prints ceiled f
print(f.ceil());