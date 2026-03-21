## Cryptography

The cryptography library in modu lets you hash strings with sha256, sha512 and blake3. And lets you hash and verify strings with bcrypt and argon2.

[CODE]
import "std/crypto";

let string = "hi";

// prints the string hashed with sha256
print(crypto.sha256(string));

// prints the string hashed with sha512
print(crypto.sha512(string));

// prints the string hashed with blake3
print(crypto.blake3(string));

// hashes the string with bcrypt
let bcrypt = crypto.bcrypt_hash(string);
print(bcrypt);

// try to make me wrong
print(crypto.bcrypt_verify("hi", bcrypt));

// hashes the string with argon2
let argon2 = crypto.argon2_hash(string);
print(argon2);

// try to make me correct
print(crypto.argon2_verify("no :(", argon2));