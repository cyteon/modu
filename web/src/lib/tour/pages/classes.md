## Classes
Classes are a core part of object-oriented programming, they are a way to create your own objects with custom functions.
A class can have an `init(...)` function, which is ran when you initialize the class, and can be used to set up any properties on the class.

Initializing the class will require the args defined in the `init(...)` function, but if you don't define an `init(...)` function then you can initialize the class without any args.

[CODE]
// creates a new class named Hello
class Hello {
    fn init() {
        print("initing...");
    }
    
    fn hello() {
        print("hi");
    }
}

// the class Counter will extend Hello, so it will also have the function hello()
class Counter extends Hello {
    // this function is optional to add, and is ran when we initialize the class
    // initializing the class will require the args defined here
    fn init(start) {
        // this will call the original init function from the Hello class 
        super.init();

        self.value = start;
    }

    // these are functions that can be ran on the class
    // functions in classes can always access self, 
    // which is the instance of the class that the function is being ran on
    fn increment() {
        self.value += 1;
    }

    fn decrement() {
        self.value -= 1;
    }

    // class functions can also return values
    fn reset() {
        let old_var = self.value;
        self.value = 0;

        return old_var;
    }
}

// initiate a new counter with a starting value of 0
let counter = Counter(0);

// you can access values on the class like any other property
print("initial value: ", counter.value);

// runs the increment function defined in the class
counter.increment();
print("after increment: ", counter.value);

// runs the decrement function
counter.decrement(); counter.decrement();
print("after decrementing twice: ", counter.value);

print("old value before reset: ", counter.reset());
print("value after reset: ", counter.value);