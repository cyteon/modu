# JSON

The JSON library has 3 functions, `.new()` and `.parse(string)`.
```rust
import "json" as json;

let object = json.new();
let string = object.to_string();
let new_object = json.parse(string);
```

### A JSON object
A json object has the following features:
```txt
object.get(key)        - gets the value of the key in the object, returns null if the key does not exist
object.has(key)        - returns true if the key exists in the object, false otherwise
object.set(key, value) - sets the value of the key in the object
object.delete(key)     - deletes the key from the object
object.to_string()     - converts the object to a json string
object.keys()          - returns an array of the keys in the object
object.values()        - returns an array of the values in the object
object[key]            - gets the value of the key in the object, returns null if the
```