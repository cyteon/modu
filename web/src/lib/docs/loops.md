# Loops

All types of loops can be broken with "break".

## Infinite Loops
These loops will keep running until you stop them.
```rust
let i = 0;

loop {
    let i = i + 1;

    if i > 10 {
        break;
    }

    print(i);
}
```
This will print the numbers 1 to 10.

## For Loops
These loops will run through an set range. These can be also be stopped prematurely with "break".
```rust
for n in 1..5 {
    print(n); 
}
```
This will print the numbers 1 to (but not including) 5. If you want to include 5, you can use `..=` instead of `..`:

## While Loops
These loops will run while the condition is true. These can be also be stopped prematurely with "break".
```rust
let x = 0;
while x < 5 {
    x = x + 1;
    print(x);
}
```
This will print the numbers 1 to (and including) 5.