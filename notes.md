# Advent of Code 2025
I'll use this file to keep track of my progress on Advent of Code 2025.

I am going to be using Rust this year. I'll try to track my progress with the language. I'm also using a new IDE: Zed. This is my fun coding project for the time being. As part of that, I won't be using AI, because coding with AI isn't fun!

# Struggling with:
Ownership, Macros

# Rust Notes
*I'm following the Rust book. https://doc.rust-lang.org/book/*
**Stopped at:** References and Borrowing (Ch4, Sec2)
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
