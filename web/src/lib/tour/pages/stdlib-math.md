## Math stdlib
The math library comes with the following methods:
- `math.rand()`
- `math.randi()`
- `math.rand_range(min, max)`
- `math.sin(v)`
- `math.cos(v)`
- `math.tan(v)`
- `math.asin(v)`
- `math.acos(v)`
- `math.atan(v)`
- `math.asind(v)`
- `math.acosd(v)`
- `math.atand(v)`
- `math.PI`
- `math.E`

[CODE]
import "std/math";

// prints a random float between 0 and 1
print(math.rand());

// prints a random valid i64 integer
print(math.randi());

// prints a random float between 1 and 10
print(math.rand_range(1, 10));

// prints the sinus of a value (in degrees) in radians
print(math.sin(30));

// prints the cosinus of a value (in degrees) in radians
print(math.sin(30));

// prints the tangent of a value (in degrees) in radians
print(math.tan(45));

// prints the arcsinus of a value in radians
print(math.asin(0.5));

// prints the arccosinus of a value in radians
print(math.acos(0.5));

// prints the arctangent of a value in radians
print(math.atan(1));

// prints the arcsinus of a value in degrees
print(math.asind(0.5));

// prints the arccosinus of a value in degrees
print(math.acosd(0.5));

// prints the arctangent of a value in degrees
print(math.atand(1));

print(math.PI);

print(math.E);