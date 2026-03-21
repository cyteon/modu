## Objects
In modu objects are created with `let obj = { ... }` and have the following methods:
- `<object>.get(key)`
- `<object>.set(key, value)`
- `<object>.has(key)`
- `<object>.delete(key)`
- `<object>.stringify()`
- `<object>.keys()`
- `<object>.values()`

[CODE]
let obj = { "value": "test" };
print(obj);

// reassign "value"
obj.value = "potato";
print(obj.get("value"));

// set a value on the object
obj.set("test", true);
// could also be done with: obj.test = true;

// check if an object has a key
print(obj.has("no")); // false

// deletes a entry from the object
print(obj.delete("value"));

// prints all keys in the object
print(obj.keys());

// prints all values in the object
print(obj.values());