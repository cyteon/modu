# JSON

The JSON library has 3 functions, `.new()`, `.to_string(object)`, and `.parse(string)`.
```rust
import "json" as json; // Alternatively `as *;` then use new(), etc

let object = json.new(); // Makes a new object
let string = object.to_string(); // Turns a object into a string
let new_object = json.parse(string); // Turns a valid JSON string into an object
```

### An JSON Object
```rust
let obj = json.new();

// Avaible functions:

// obj.set(key, value)
obj.set("name", "test");

// obj.has(key)
obj.has("name"); // true

// obj.get(key)
obj.get("name"); // test
// You could also do
obj.name // test

// obj.delete(key)
obj.delete("name");

print(obj);
// {  }

```