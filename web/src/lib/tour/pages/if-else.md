## If / Else
If statements will run the code of a branch if the condition is true, and if all are false the else branch will run (if it exists).

The following operators can be used in conditions:
- `==` `!=` `>` `<` `>=` `<=`
- `and`, `or`, `!`
- `in`, `not in`

[CODE]
let x = 5;

// the condition is true so the first branch will run
if x > 0 {
    print("true");
}

// the condition is false so the else branch will run
if x < 2 {
    print("false");
} else {
    print("something else");
}

// the condition is false but the else if condition is true so the else if branch will run
if false {
    print("false");
} else if x == 5 {
    print("true");
} else {
    print("false");
}

// both sides have to be true for the branch to run
if (x != 0) and (x < 10) {
    print("true");
}

// either side can be true for the branch to run
if (x == 5) or (x == 10) {
    print("true");
}

// runs as 5 is a value of the array [4, 5, 6]
if x in [4, 5, 6] {
    print("true");
}

// runs as 5 is not a value of the array [1, 2, 3]
if x not in [1, 2, 3] {
    print("true");
}

// we use ! to negate a condition
if !false {
    print("true");
}