## Encoding
The encoding library in modu lets you encode and decode base64 and base16!

[CODE]
import "std/encoding";

let string = "Hello, World!";

// prints the string encoded as base64
print(encoding.encode_base64(string));

// prints the string encoded as base16
print(encoding.encode_base16(string));

// prints the decoded base64 string
print(encoding.decode_base64("SGVsbG8sIFdvcmxkIQ=="));

// prints the decoded base16 string
print(encoding.decode_base16("48656c6c6f2c20576f726c6421"));