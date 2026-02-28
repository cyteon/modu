# Encoding

The encoding package currently features base64 and base16 encoding and decoding functions.

## Base64
```rust
import "encoding" as encoding;

let encoded = encoding.encode_base64("test");
print(encoded); // Outputs: "dGVzdA=="

let decoded = encoding.decode_base64("dGVzdA==");
print(decoded); // Outputs: "test"
```

## Base16
```rust
import "encoding" as encoding;

let encoded = encoding.encode_base16("test");
print(encoded); // Outputs: "74657374"

let decoded = encoding.decode_base16("74657374");
print(decoded); // Outputs: "test"
```