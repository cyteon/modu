## JSON
Modu comes with a JSON library, but its only used for parsing json.

[CODE]
// imports std/json as "json"
// you could also do it like
import "std/json";

// you can change me!
let str = "{ \"test\": 1 }";

// json.parse returns an object
// then we access the test property on that object
print(json.parse(str).test);