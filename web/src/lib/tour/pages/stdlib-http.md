## HTTP
The HTTP package does not work in browsers, but here is a demonstration of what it has

[CODE]
import "std/http";

// sends a get request and prints the body
print(http.get("https://example.com").body);

let headers = { "User-Agent": "Modu v3 HTTP" };
let body = "modu!";

// sends a post request with a body and headers 
// and prints the entire response object
// headers and body is optional, but they have to be
// provided in the order url, body, headers
print(http.post("https://httpbin.org/post", body, headers));

// sends a put request with a body and headers
print(http.put("https://httpbin.org/put", body, headers));

// sends a patch request with a body and headers
print(http.patch("https://httpbin.org/patch", body, headers));

// sends a delete request with headers
print(http.delete("https://httpbin.org/delete", headers));
