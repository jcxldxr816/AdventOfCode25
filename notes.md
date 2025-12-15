# Advent of Code 2025
I'll use this file to keep track of my progress on Advent of Code 2025.

I am going to be using Rust this year. I'll try to track my progress with the language. I'm also using a new IDE: Zed. This is my fun coding project for the time being. As part of that, I won't be using AI, because coding with AI isn't fun!
## Progress
6/24 stars
- Day 1 (Part 1 & 2) Complete
- Day 2 (Part 1 & 2) Complete
- Day 3 (Part 1 & 2) Complete

# Struggling with:
Ownership, Macros
rust-analyzer seems to only work with cargo projects.

# Rust Notes
*I'm following the Rust book. https://doc.rust-lang.org/book/*
**Ready for:** Ch7: Packages, Crates, and Modules

## Tools
- cargo - Dependency manager & build tool
  - ```cargo new [name]``` - create a new project
  - ```init```
  - ```build``` - creates a debug build by default
    - ```--release``` flag. compiles with optimizations. takes longer to compile, but code will run faster.
  - ```run``` - compile and run (also debug build by default) **[BETTER CHOICE]**
  - ```check``` - checks for compile-ability, faster than build
  - ```update``` - ignore Cargo.lock and look for new versions of dependencies
- ```rustup doc``` to view the docs (even offline)
- rustc - compiler
## Syntax
### Data Types
#### Scalar Types
(Scalar types represent single values)
- Integers *(i/u + size(bits))*
  - You can use _ as a visual separator (1,000 = 1_000)
  - You can use different wrapping modes for handling over/underflows
- Floating point *(f32, f64)*
  - Chapter 3 did not cover how division with integers should be done
- Booleans *(bool)*
  - true/false
- Characters *(char)*
  - single quotes
#### Compound Types
(Grouping multiple values into one type)
There are two primitive compounds: tuples and arrays.
- Tuple
  - fixed length
  - can be destructured easily
    - ```let (x,y,z) = tup;```
  - values can be accessed by index
    - ```let one = tup.2;```
  - empty tuple = a "unit"
- Array
  - fixed length
  - type annotation example (type, numOfElements)
    - ```let a: [i32, 5]```
## Behavior
- variables are immutable by default. use ```mut``` to change that. variables passed by reference are also immutable by default.
- variable shadowing: often used when converting the type of a value
- Rust doesn't have a null value
## Functions vs Macros
- function syntax is pretty similar to other languages
  - ```println("msg")```
  - parameters must be annotated with their types
- macros generate code to extend Rust syntax
  - ```println!("msg")```
  - Testing sql query at compile time example
- statements vs expressions
  - statements are instructions that don't return a value, while expressinos evaluate and return a value
    - ```x+1``` - expression, returns a value
    - ```x+1;``` - statement, won't return anything
  - you can't do ```x=y=6```, since assignment is a statement
  - the last expression in a function is **returned implicitly**
  - you must annotate return types for functions
## Control Flow
*(conditionals and loops)*
- you can label loops: ```'loop_name: loop {``` (must start with single quote)
- loop, while, for (in)
  - ```for number in (1..4) {``` - range syntax
## Development Process
- add dependencies before "using/importing" into file?
- good practice to include Cargo.lock in source control checkins
## Ownership
**Ch4, Sec1**
Ensures memory safety without using a garbage collector. Ownership doesn't slow down programs during runtime. The compiler checks a set of rules.
### Remember:
- Stack
  - LIFO
  - All data on the stack must be a known, fixed size
- Heap
  - Unknown/dynamic size data storage
  - Data is not actually "allocated" on the stack, but *is* in the heap.
  - Takes more work to store data in heap, since space has to be allocated
### Rules
To my understanding, scope works like it does in other languages.
Memory that has been allocated for a variable will be automatically returned (by the ```drop``` function) when variable reaches the end of its scope.

"Variable" in this case, refers to heap variables, or variables with unknown sizes

When a "shallow copy" is made of a variable (String example), the original variable is considered invalid to prevent a **double free error**. 

*It's not actually a shallow copy, but rather a "move"*. Memory is only cleared when the copy/second variable goes out of scope.

Rust doesn't make "deep copies", so any automatic copying that occurs is inexpensive.

If a variable is reassigned, the original value in memory is dropped

```.clone()``` can be used to perform a deep copy, but is expensive

Data types (that are stored on the stack) either have the ```Copy``` trait or the ```Drop``` trait. For example, ints are annotated with 'Copy', signifying that there's no difference between deep and shallow copies (since the size is known/fixed).

These traits' behavior apply when passing variables to functions as well. *Copy* variables don't have any problems, but *Drop* variables will be made invalid after being passed into a function.

Ownership can be transferred when returning values as well.

References can be used to avoid repetitive ownership changes.
### References and Borrowing
References are like pointers, but they will always point to a valid value of a specific type.
- Remember: Reference: &, Dereference: *
Variables passed by reference won't be owned, but borrowed.
By default, refrences are immutable. Variables can only be passed as mutable references one at a time.
You can't have a mutable reference while there is an existing immutable reference.
#### Dangling References
The data will not go out of scope before the reference does.
Example: returning a reference to a variable that was created within the function.

You can either have one mutable reference, or any number of immutable references at a time.
References must always be valid.
### The Slice Type
A slice is a kind of reference, so it doesn't have ownership.
Slices allow you to reference a contiguous sequence of elements in a collection.
String Slice (&str) Example: "s = hello world"
```let world = &s[6..11];```
A more general slice: &[i32]
There are plenty of slice types
## Structs
### Field Init Shorthand:
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true, //init with value true
        username,
        email, //same as "email: email,"
        sign_in_count: 1,
    }
}
```
### Struct Update:
*useful when copying a portion of fields from another instance*
```rust
let user2 = User {
    email: String::from("example@email.com"),
    ..user1 //.. syntax to copy remaining fields
}
```
This *moves* the data, so in this example, we can't use user1 'String' fields anymore.
### Tuple Structs
Allows you to differentiate tuples with "names"
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0); //these are different types
}
```
### Unit-Like Structs
Structs without fields = units

These are useful when you need to implement a trait on a type, but don't have any data to store.
### Outer Attribute Example
```rust
#[derive(Debug)] //adding Debug trait for better struct/field-value output with 'println!'
struct Rectangle {...}
```
### Methods
- Like functions (defined with 'fn' as well), but are defined within context of a struct, enum, or trait.
- First parameter is always self
- Defined within the context of a struct, using 'impl' implementation block.
- Methods can take ownership of self, borrow immutable, or borrow mutably.
```rust
struct Rectangle {
    //...
}
impl Rectangle {
    fn area(&self) -> u32 { //&self is short for self: &self
        self.width * self.height
    }
}
```
- getters/setters are not implemented automatically in Rust
- Rust uses automatic referencing/dereferencing, so you don't need to use '.' for direct method calls and '->' for other method calls.
#### Associated Functions
- Like methods, they are defined in the 'impl' block, but do not take 'self' as the first parameter.
  - multiple 'impl' blocks are allowed
Example:
```rust
impl Rectangle {
    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }
}
//...
let sq = Rectangle::square(3);
```
'::' syntax is used for namespaces *and* associated functions.
## Enumerations
"enums" allow you to define types by enumerating its possible variants.
Structs allow grouping data, enums are sets of possible values.

Example
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
- Enums can be used to store values, or not :P
- You can store enums in structs, and structs in enums
- variants can have named fields as well: ```Move {x: i32, y:  i32},```
### Methods
Enums support methods as well. They are defined in the same way as struct methods.
```rust
let m = Message::Write(String::from("hello")); //Enum::Variant()
m.call(); //.Method()
```
### 'Option' Enum Example
Common in the standard library, encodes the situation in which a value could be something, or nothing. Consider requesting an item in a non-empty list vs an empty list.

Rust doesn't have a null value, but it does have the Option enum.
```rust
enum Option<T> {
    None,
    Some(T),
}
```
*<T> generic type parameter*
If you have an ``Option<T>``, you need to handle each variant

## Pattern Matching
Great way to process variants/arms of an enum. Like a switch in other langs.
```rust
match coin {
    Coin::Penny => 1, //first arm
    Coin::Nickel => 5, //another arm
    //arms consist of ``pattern => code``
}
```
``if`` needs to evaluate to a boolean, but ``match`` doesn't.

Pattern matching can be used to bind values to variables. **See the quarter example in Ch6, Sec2**
*Also see the ``plus_one(x)`` example on matching ``Option<T>`` variants.*

Matches are exhaustive. The arms must cover all possibilities.
However, you can use catch-all patterns.
```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // other => move_player(other),
    _ => reroll(),
}
```
You can use ``match`` and ``let if`` to extract and use values from enums.
**Skipped over ``if let``` and ``let else`` in 'Ch6, Sec3' **

# Random Notes
```println!("{var:?}")``` Debug trait for formatting output
