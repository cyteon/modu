# Infinite Loops
Infinite loops keep running until you stop them.

[CODE]
let x = 0;

// runs until stopped
loop {
    print(x);
    x += 1;

    if x > 5 {
        // stops the loop
        break;
    }
}