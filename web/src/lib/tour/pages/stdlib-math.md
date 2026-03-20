## Math stdlib
The math library comes with the following methods:
- `math.rand()`
- `math.randi()`
- `math.sin(v)`
- `math.cos(v)`
- `math.tan(v)`
- `math.PI`
- `math.E`

[CODE]
import "std/math";

// prints a random float between 0 and 1
print(math.rand());

// prints a random valid i64 integer
print(math.randi());

// prints the sinus of a value (in degrees) in radians
print(math.sin(30));

// prints the cosinus of a value (in degrees) in radians
print(math.sin(30));

// prints the tengent of a value (in degrees) in radians
print(math.tan(45));