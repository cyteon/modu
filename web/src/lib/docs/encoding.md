# Encoding
> Introduced in Modu v1.2.0

The encoding package currently features base64 and base16 encoding and decoding functions.

## Base64
```rust
import "encoding" as encoding;

let encoded = encoding.base64_encode("test");
print(encoded); // Outputs: "dGVzdA=="

let decoded = encoding.base64_decode("dGVzdA==");
print(decoded); // Outputs: "test"
```

## Base16
```rust
import "encoding" as encoding;

let encoded = encoding.base16_encode("test");
print(encoded); // Outputs: "74657374"

let decoded = encoding.base16_decode("74657374");
print(decoded); // Outputs: "test"
```