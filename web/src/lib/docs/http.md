# HTTP

The http library is used to interact with the internet.

## The http request result object
```
ok          - boolean
status      - integer
status_text - string
headers     - object
body        - string
```

## Example get request
```rust
import "http" as http;

let result = http.get("https://httpbin.org/get");
print("Status: ", result.status, " - ", result.status_text);
print("\nHeaders: ", result.headers);
print("\nBody:\n", result.body);
```

## Example post request
```rust
import "http" as http;
import "json" as json;

let data = json.new();
data.set("name", "modu");

let result = http.post("https://httpbin.org/post", data.to_string());
print("Status: ", result.status, " - ", result.status_text);
print("\nHeaders: ", result.headers);
print("\nBody:\n", result.body);
```

## Example put request
```rust
import "http" as http;
import "json" as json;

let data = json.new();
data.set("name", "modu");

let result = http.put("https://httpbin.org/put", data.to_string());
print("Status: ", result.status, " - ", result.status_text);
print("\nHeaders: ", result.headers);
print("\nBody:\n", result.body);
```

## Example patch request
```rust
import "http" as http;
import "json" as json;

let data = json.new();
data.set("name", "modu");

let result = http.patch("https://httpbin.org/patch", data.to_string());
print("Status: ", result.status, " - ", result.status_text);
print("\nHeaders: ", result.headers);
print("\nBody:\n", result.body);
```

## Example delete request
```rust
import "http" as http;
import "json" as json;

let result = http.delete("https://httpbin.org/delete");
print("Status: ", result.status, " - ", result.status_text);
print("\nHeaders: ", result.headers);
print("\nBody:\n", result.body);
```