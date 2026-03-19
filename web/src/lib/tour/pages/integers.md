## Integers
In modu, integers are i64 numbers, and they come with the following built in functions:
- `<int>.max(b)`
- `<int>.min(b)`
- `<int>.abs()`
- `<int>.sqrt()`

[CODE]
let i = 5;
print(i);

// prints 4 if its smaller than i, otherwise it prints i
print(i.min(4));

// prints 6 if its bigger than i, otherwise it prints i
print(i.max(6));

// prints the absolute value of i
print(i.abs());

// prints the sqrt of i
print(i.sqrt());