# Libraries

Here is how to use, and how to make libraries. \
The official package repo is https://mpm.cyteon.dev, but you can enter a custom one when doing **modu login**. \
The package repo code is open source at https://github.com/cyteon/modu-packages

## Using libraries
Installing libraries is as simple as finding the package on [the official repo](https://mpm.cyteon.dev), or another, list will be at end.

1. Initialize a new modu project 
```bash
$ modu init
```

2. Install the package
```bash
$ modu install package_name
```

3. Import the package
```rust
import "package_name" as package_name;

package_name.a_function();
```

## Making libraries
Making libraries is really simple, first make sure you have initialized a new project. \
If you havent already, run `modu init` and say Y when it asks if its an library. \
If you already have, and its not a library, make a `lib.modu` file, as that is what gets imported when you import the package.

The `lib.modu` file will export anything defined in the file, this will export the function `hi()`
```rust
// lib.modu
fn hi() {
    print("Hello, World!"); // This is good
}
```

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
https://mpm.cyteon.dev | Owner: [cyteon](https://github.com/cyteon)     - Offical

Want to list yours? Make a PR to modify [this file](https://github.com/cyteon/modu/blob/main/web/src/lib/docs/libraries.md)