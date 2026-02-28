# HTTP

The http library is used to interact with the internet.

For get and delete requests, you can pass custom headers, these are optional. \
For post, put and patch requests, you can pass a body and custom headers, both are optional but have to be in the correct order (body first, then headers).
If you want to pass headers but no body, pass an empty string as the body. \
The body is always a string, and headers should be a object.

## The http request result object
```txt
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

let headers = json.new();
headers.set("Content-Type", "application/json");

let result = http.post("https://httpbin.org/post", "", headers);
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

let headers = json.new();
headers.set("Content-Type", "application/json");

let result = http.put("https://httpbin.org/put", data.to_string(), headers);
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