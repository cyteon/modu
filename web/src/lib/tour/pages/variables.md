## Variables
Variables are assigned with `let <name> = <value>;` and can be reassigned with `<name> = <value>;`
or one of the following operators: `+=`, `-=`, `*=`, `/=`, `%=`.

Constants can be declared with `const <name> = <value>;` and cannot be reassigned.

[CODE]
// declare the variable 'a' with the value 1
let a = 1;

// multiply the variable 'a' with 2
a *= 2;

print(a);

const MESSAGE = "hi!";
// try to uncomment me:
// MESSAGE = "bye!"