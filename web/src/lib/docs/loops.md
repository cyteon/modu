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
This will print the numbers 1 to 5.