## If / Else
If statements will run the code of a branch if the condition is true, and if all are false the else branch will run (if it exists).

[CODE]
// the condition is true so the first branch will run
if true {
    print("true");
}

// the condition is false so the else branch will run
if false {
    print("false");
} else {
    print("something else");
}

// the condition is false but the else if condition is true so the else if branch will run
if false {
    print("false");
} else if true {
    print("true");
} else {
    print("false");
}