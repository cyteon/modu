# Break & Countinue
Break and continue is used to control the flow of loops. `break;` will exit the loop, while `continue;` will skip to the next iteration of the loop.

[CODE]
let x = 0;

loop {
    print(x);
    x += 1;

    if x > 5 {
        break;
    }

    continue;

    print("this wont print");
}