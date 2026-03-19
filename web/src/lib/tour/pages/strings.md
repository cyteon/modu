## Strings
In modu strings have the following builtins
- `<string>.len()`
- `<string>.split(delimiter)`
- `<string>.replace(old, new)`
- `<string>.trim()`
- `<string>.to_upper()`
- `<string>.to_lower()`
- `<string>.starts_with(str)`
- `<string>.ends_with(str)`
- `<string>.chars()`

[CODE]
let s = "hi!"; // try to change me!
print(s);

// prints the length of the string
print(s.len());

// splits the string into an array based on the delimiter
print(s.split("i")); // will return ["h", "!"], but wont modify the string

// replaces a pattern in the string
print(s.replace("hi", "bye")); // will return "bye!" while leaving the original unmodified

// trims the string
print(s.trim());

// returns the string in uppercase while leaving the original unmodified
print(s.to_upper());

// returns the string in lowercase while leaving the original unmodified
print(s.to_lower());

// returns true if the string starts with the arg
print(s.starts_with("hi"));

// returns true if the string ends with the arg
print(s.starts_with("no"));

// returns an array of all the chars in the string
print(s.chars());