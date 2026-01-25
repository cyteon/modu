# JSON
> Introduced in Modu v0.5.4

The JSON library has 3 functions, **.new()**, **.stringify(object)**, and **.parse(string)**.
```rust
import "json" as json; // Alternatively `as *;` then use new(), etc

json.new(); // Makes a new object
json.stringify(object); // Turns a object into a string
json.parse(string); // Turns a valid JSON string into an object
```

### An JSON Object
```rust
let obj = json.new();

// Avaible function:

// obj.set(key, value)
obj.set("name", "Modu User");

// obj.has(key)
obj.has("name"); // true

// obj.get(key)
obj.get("name"); // Modu User
// You could also do
obj.name // Modu User

// obj.delete(key)
obj.delete("name");

print(obj);
// {  }

```