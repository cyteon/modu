# Cryptography

The cryptography package currently features the hasing functions SHA256, SHA512, and BLAKE3. \
And it also features password hashing algorithms like bcrypt, argon2 and scrypt.

## SHA256
```rust
import "std/crypto";

print(crypto.sha256("test"));
```

## SHA512
```rust
import "std/crypto";

print(crypto.sha512("test"));
```

## BLAKE3
```rust
import "std/crypto";

print(crypto.blake3("test"));
```

## Bcrypt
```rust
import "std/crypto";

let hashed = crypto.bcrypt_hash("password");
print(hashed);

let valid = crypto.bcrypt_verify("password", hashed);
print(valid); // outputs: true
```

## Argon2
```rust
import "std/crypto";

let hashed = crypto.argon2_hash("password");
print(hashed);

let valid = crypto.argon2_verify("password", hashed);
print(valid); // outputs: true
```