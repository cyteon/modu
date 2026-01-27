# Cryptography
> Introduced in Modu v1.2.0

The cryptography package currently features the hasing functions SHA256, SHA512, and BLAKE3. \
And it also features password hashing algorithms like bcrypt, argon2 and scrypt.

## SHA256
```rust
import "crypto" as crypto;

print(crypto.sha256("test"));
```

## SHA512
```rust
import "crypto" as crypto;

print(crypto.sha512("test"));
```

## BLAKE3
```rust
import "crypto" as crypto;

print(crypto.blake3("test"));
```

## Bcrypt
```rust
import "crypto" as crypto;

let hashed = crypto.bcrypt_hash("password");
print(hashed);

let valid = crypto.bcrypt_verify("password", hashed);
print(valid); // outputs: true
```

## Argon2
```rust
import "crypto" as crypto;

let hashed = crypto.argon2_hash("password");
print(hashed);

let valid = crypto.argon2_verify("password", hashed);
print(valid); // outputs: true
```

## Scrypt
This is slow asf, makes it more secure ig.
```rust
import "crypto" as crypto;

let hashed = crypto.scrypt_hash("password");
print(hashed);

let valid = crypto.scrypt_verify("password", hashed);
print(valid); // outputs: true
```