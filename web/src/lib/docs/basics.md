# Basics

**Note: Semicolons are required!**

## Variables

Variables can be defined, and redefined with 'let'. \
Like this:

```rust
let a = 1;
let b = "a";
let c = false;
let d = a;
```

Variables can also be redefined with `=`, `+=`, `-=`, `*=`, `/=` and `%=`.
```rust
let a = 1;
a += 1; // a is 2
a *= 3; // a is 6
```

## User Input
User input can be gotten with the built-in function `input()`, which takes any amount of arguments
```rust
let name = input("What is your name? ");

print("Hello, ", name, "!");
```

## Functions

Functions are defined with the `fn` keyword, then with arguments inside of parentheses. \
There is currently no support for default values, and modu will return an error if you provide the wrong number of arguments.

```rust
fn yap(msg) {
    print(msg);
}

yap("Hello, World!");

// Outputs
//
// Hello, World!
```

Functions defined in a file, can be also be accessed in other files when imported, see [imports](imports).

## Conditions

Modu has the following operators: `==`, `!=`, `>`, `&lt;`, `>=` and `<=` \
If the condition given returns true, the code inside of the brackets is ran.

```rust
if 1 == 1 {
    print("yes");
} else if 1 == 2 {
    print("never");
} else {
    print("we are cooked");
}

if 1 != 2 {
    print("duh");
}

// Outputs
//
// yes
// duh
```

You can also use conditions to a check if a value is not null or false like this, instead of `== true`:
```rust
if a {
    print("a exists and is not null");
}
```