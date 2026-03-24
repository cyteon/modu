## Error Handling
Error handling is done with try/catch blocks. And you can throw errors using `error(msg)`.

[CODE]
// try/catch is used for error handling
try {
    // throws an error, this would normally crash the program
    error("this exploded");
} catch { // the error variable is optional
    print("error occurred");
}

try {
    print("smth" + 5);
} catch error_variable {
    // the error variable can be named wtv
    // and is the error that happened in the try block as a string
    print("error occurred: " + error_variable);
}