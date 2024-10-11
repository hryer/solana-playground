# Summary / Notes

## Structures Project
`lib.rs`: This file acts as the main entry point of the program and typically contains high-level logic and the core definitions of the program’s state (like `structures and data types`). It defines the general setup for how the program operates.
`instructions.rs`: This file handles the instructions (or actions) that the program can execute. Instructions are the `specific commands` that clients send to the program to tell it what to do, such as incrementing a counter, transferring tokens, or performing any other operation.


## TLDR;
### What is a Macro in Rust?
**Definition**: A macro in Rust is a way to write code that writes other code (also known as metaprogramming). Macros allow you to generate code based on specific patterns, which helps reduce code duplication and enables more powerful abstractions.
**Purpose**: Macros are used to automate repetitive code patterns, generate boilerplate code, and allow more flexible syntax compared to functions.

**examples without macros in rust:**

```rust

use std::fmt; // Importing the standard formatting library

// Defining the struct
struct MyStruct {
    name: String,
    age: u32,
}

// Manually implementing the Debug trait for the struct
impl fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyStruct {{ name: \"{}\", age: {} }}", self.name, self.age)
    }
}

fn main() {
    let person = MyStruct {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{:?}", person); // This will print "MyStruct { name: "Alice", age: 30 }"
}
```

**example with macros in rust:**

```rust
// Using the #[derive(Debug)] macro to automatically implement the Debug trait
#[derive(Debug)]
struct MyStruct {
    name: String,
    age: u32,
}

fn main() {
    let person = MyStruct {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{:?}", person); // This will print "MyStruct { name: "Alice", age: 30 }"
}

```
