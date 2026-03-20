# For Loops
For loops can iterate on ranges (`x..y` / `x..=y`) and arrays.

[CODE]
// prints 0, 1, 2, 3, 4. 
// if you used ..= it would print 5 as that means inclusive
// try to change it!
for i in 0..5 {
    print(i);
}

// prints each variable in the array
for v in ["a", "b", "c"] {
    print(v);
}