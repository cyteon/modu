# Libraries
Here is how to use, and how to make libraries. \
The official package repo is https://modu-packages.vercel.app, but you can enter a custom one when doing **modu login**. \
The package repo code is open source at https://github.com/cyteon/modu-packages

## Using libraries
Installing libraries is as simple as finding the package on [the official repo](https://modu-packages.vercel.app), or another, list will be at end.

1. Initialize a new modu project 
```bash
$ modu init
```

2. Install the package
```bash
$ modu install package_name # example: modu install basic_loops
```

3. Import the package
```rust
import "package_name" as package_name // or anything else

package_name.cool_function();
```

## Making libraries
Making libraries is really simple, first make sure you have initialized a new project. \
If you havent already, run **modu init** and say Y when it asks if its an library. \
If you already have, and its not a library, make a **lib.modu** file, as that is what gets imported when you import the package.

The **lib.modu** file should **NOT** run any functions on the root of the file, example
```rust
// lib.modu
print("Hello, World!"); // BAD, DO NOT DO THIS!
```

Rather, it should be something like:
```rust
// lib.modu
fn hi() {
    print("Hello, World!"); // This is good
}
```
So that code is NOT ran when the library is imported, and only when the user wants something to happen, for example calling **hi()**.

Now, once you have coded in the functionality you want your library to have, its time to publish it! \
First make sure you have filled out package name, version and description (optional) \
And you can also write a README.md that will be displayed on the package page, which can for example include how to get started!

To publish your package just follow theese two steps:
1. Log into the modu CLI:
```bash
$ modu login
```

2. Publish the package
```bash
$ modu publish
```

## Package repos
https://modu-packages.vercel.app | Owner: [cyteon](https://github.com/cyteon)     - Offical \
https://modu.gizzy.pro/          | Owner: [GizzyUwU](https://github.com/GizzyUwU) - Unoffical

Want to list yours? Make a PR to modify [this file](https://github.com/cyteon/modu/blob/main/web/src/lib/docs/libraries.md)