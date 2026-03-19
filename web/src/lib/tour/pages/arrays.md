## Arrays
In modu strings have the following methods:
- `<array>.len()`
- `<array>.push(value)`
- `<array>.pop()`
- `<array>.join("delimiter")`
- `<array>.min()`
- `<array>.max()`
- `<array>.reverse()`
- `<array>.sort()`

[CODE]
let arr = [1, 2, 3];

// prints the length of the array
print(arr.len());

// pushes a value to the array
arr.push(5);
print(arr[3]);

// removes the last value of the array and returns it
print(arr.pop());
print(arr);

// prints a string with all array elements joined with ", "
print(arr.join(", "));

// prints the smallest value in the array
print(arr.min());

// prints the biggest value in the array
print(arr.max());

// prints the reversed array
print(arr.reverse());

// prints the sorted array
print(arr.sort());