# HTTP
> Introduced in Modu v1.0.0

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

let result = http.get("https://example.com");
print("Status: ", result.status, " - ", result.status_text);
print("\nHeaders: ", result.headers);
print("\nBody:\n", result.body);
```