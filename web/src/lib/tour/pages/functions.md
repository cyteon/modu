## Functions
Functions are decleared with `fn <name>(<args>) { <body> }`.
Args you define will need to be passed to the function and are accessible in the functions body.
You can use `return;` to return prematurely and/or return a value.

[CODE]
// Decleares a function 'add' that takes the args 'a' and 'b'
fn add(a, b) {
    return a + b;
}

print(add(1, 2));

// Try to uncomment me
// print(add(5));