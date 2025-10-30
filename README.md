# Rust Playgrounds

A place to learn the Rust language.


## Introduction

**Goal:** To gain a solid understanding of Rust's core principles—ownership, borrowing, and safety—and to become comfortable building robust, efficient, and enjoyable command-line tools and applications. This curriculum is designed for the curious hobbyist who wants to learn for fun and build interesting personal projects.

**Philosophy:** Learn by doing. The Rust compiler is your personal tutor, not your critic. We will learn to embrace its detailed error messages as helpful guidance on our path to writing correct and safe code. Consistency over intensity is key; even 20-30 minutes of focused daily practice will build a strong foundation. The goal is enjoyment and discovery.

**Prerequisites:** A solid foundation in C (understanding variables, loops, functions, pointers, and manual memory management with `malloc`/`free`). You'll also need a text editor (like VS Code, Sublime Text, or Vim) and access to a terminal.

**The C Parallel:** Your C knowledge is a superpower here! It gives you a deep appreciation for *why* Rust makes certain design choices. We will constantly compare Rust's approach (e.g., Ownership, `Result`, `Option`) to C's (`malloc`/`free`, return codes, `NULL`) to build a rich, comparative understanding of systems programming.

**Your Source of Truth:** Use this document as your guide. Track your progress, revisit concepts, and don't be afraid to spend an extra day on a topic that you find challenging or particularly interesting.

## Curriculum Phases Overview

*   **Phase 0: Liftoff (Day 1)** - Installing the toolchain and launching our first program with Cargo.
*   **Phase 1: Rust Fundamentals (Days 2-17)** - Variables, data types, functions, and control flow—the building blocks of any language.
*   **Phase 2: The Ownership System (Days 18-37)** - The heart of Rust. We'll take our time to deeply understand ownership, borrowing, and slices, the concepts that provide memory safety without a garbage collector.
*   **Phase 3: Custom Data Types (Days 38-50)** - Structuring data with `struct`s and modeling state with Rust's powerful `enum`s and the `match` control flow operator.
*   **Phase 4: Organizing and Collecting Data (Days 51-65)** - How to organize code into modules and how to use Rust's standard collections like vectors and hash maps.
*   **Phase 5: Robust Error Handling (Days 66-73)** - Moving from C's error codes and `NULL` to Rust's expressive `Result` and `Option` enums for building resilient programs.
*   **Phase 6: Abstraction and Generics (Days 74-90)** - Writing flexible and reusable code with generics, traits (similar to interfaces), and lifetimes.
*   **Phase 7: Building Real Tools (Days 91-99)** - Consolidating our knowledge by exploring closures, iterators, and building a complete command-line tool.

---

## Detailed Steps

---

### **Phase 0: Liftoff (Day 1)**

- ✅ **Day 1: Setup & "Hello, Cargo!"**
    - **Topic:** Installing the Rust toolchain (`rustup`) and using `cargo` to create and run your first project.
    - **Exercise:** Use `cargo new hello_rust` to create a new project. Change the `main.rs` file to print "Hello, Rust!". Run it using `cargo run`.
    - **Hint:** `cargo run` compiles and executes your code in one step.
    - **Book:** `The Rust Programming Language`, Chapter 1

---

### **Phase 1: Rust Fundamentals (Days 2-17)**

- ✅ **Day 2: Variables and Mutability**
    - **Topic:** Declaring variables with `let` and making them mutable with `let mut`.
    - **Exercise:** Create a program that declares an immutable variable `x` with a value of 5. Print it. Then try to assign a new value to `x` and observe the compiler error. Change `let` to `let mut` to make it work.
    - **Hint:** The compiler error message will tell you exactly what you need to do!
    - **Book:** Chapter 3.1
- ✅ **Day 3: Constants**
    - **Topic:** Understanding the difference between immutable variables and `const`ants.
    - **Exercise:** Declare a constant for the number of seconds in an hour and use it in a calculation.
    - **Hint:** Constant names are conventionally in `UPPER_SNAKE_CASE`.
    - **Book:** Chapter 3.1
- ✅ **Day 4: Shadowing**
    - **Topic:** Re-declaring a variable with `let` to "shadow" its previous value, allowing for type changes.
    - **Exercise:** Declare a variable `spaces` that is a string of " ". Then, shadow it with a new variable, also named `spaces`, that holds the *number* of spaces.
    - **Hint:** You'll use `let` twice for the same variable name.
    - **Book:** Chapter 3.1
- ✅ **Day 5: Integer Types**
    - **Topic:** Exploring Rust's signed (`i8` to `i128`) and unsigned (`u8` to `u128`) integer types.
    - **Exercise:** Declare variables of different integer types (e.g., `u8`, `i32`) and print them.
    - **Hint:** The default integer type is `i32`.
    - **Book:** Chapter 3.2
- ✅ **Day 6: Floating-Point, Boolean, and Character Types**
    - **Topic:** Using `f32`/`f64` for decimals, `bool` for true/false, and `char` for single Unicode characters.
    - **Exercise:** Create a program with one variable of each type (`f64`, `bool`, `char`) and print their values.
    - **Hint:** `char` literals use single quotes, like `'z'`.
    - **Book:** Chapter 3.2
- ✅ **Day 7: The Tuple Type**
    - **Topic:** Grouping a fixed number of values of different types into one compound type.
    - **Exercise:** Create a tuple `(i32, f64, u8)`. Use destructuring and index access (`.0`) to print its elements.
    - **Hint:** `let (x, y, z) = my_tuple;` is a great way to get elements out.
    - **Book:** Chapter 3.2
- ✅ **Day 8: The Array Type**
    - **Topic:** Working with fixed-size collections where every element must have the same type.
    - **Exercise:** Create an array of the first 5 months of the year. Try to access an element by its index.
    - **Hint:** Accessing an index out of bounds will cause a `panic!`.
    - **Book:** Chapter 3.2
- ✅ **Day 9: Functions with Parameters**
    - **Topic:** Defining functions that accept input values (arguments).
    - **Exercise:** Write a function `print_value(x: i32)` that takes an integer and prints it. Call this function from `main`.
    - **Hint:** You must declare the type of each function parameter.
    - **Book:** Chapter 3.3
- ✅ **Day 10: Functions with Return Values**
    - **Topic:** Defining functions that return a value using the `->` syntax.
    - **Exercise:** Write a function `plus_one(x: i32) -> i32` that takes an integer and returns that integer plus one.
    - **Hint:** The last expression in a function is implicitly returned (no semicolon needed).
    - **Book:** Chapter 3.3
- ✅ **Day 11: `if`/`else` Expressions**
    - **Topic:** Branching your code's execution based on a condition.
    - **Exercise:** Write a program that checks if a number is divisible by 2 and prints "even" or "odd".
    - **Hint:** The modulo operator is `%`.
    - **Book:** Chapter 3.5
- ✅ **Day 12: Using `if` in a `let` Statement**
    - **Topic:** Using the result of an `if`/`else` expression to assign a value to a variable.
    - **Exercise:** Use an `if` expression to assign a value to a variable `condition` based on whether a number is greater than 5.
    - **Hint:** The types of the values returned from each branch of the `if` must be the same.
    - **Book:** Chapter 3.5
- ✅ **Day 13: The Unconditional `loop`**
    - **Topic:** Using `loop` to create an infinite loop, and `break` to exit it.
    - **Exercise:** Write a `loop` that counts up from 0 and `break`s when the counter reaches 5.
    - **Hint:** You can return a value from a `loop` using `break some_value;`.
    - **Book:** Chapter 3.5
- ✅ **Day 14: The `while` Loop**
    - **Topic:** Looping as long as a condition is true.
    - **Exercise:** Re-implement the countdown from Day 13 using a `while` loop.
    - **Hint:** Make sure the condition eventually becomes false to avoid an infinite loop!
    - **Book:** Chapter 3.5
- ✅ **Day 15: The `for` Loop**
    - **Topic:** Iterating over elements of a collection, like an array.
    - **Exercise:** Create an array `a = [10, 20, 30, 40, 50]` and use a `for` loop to print each element.
    - **Hint:** The `for element in a` syntax is clean and safe.
    - **Book:** Chapter 3.5
- ✅ **Day 16: `for` Loops with Ranges**
    - **Topic:** Using a `for` loop with a `Range` to execute code a certain number of times.
    - **Exercise:** Create a `for` loop that prints numbers 1 through 5 using a range (`1..6`).
    - **Hint:** Use `.rev()` on a range to reverse its direction.
    - **Book:** Chapter 3.5
- ✅ **Day 17: Fahrenheit to Celsius Project**
    - **Topic:** Consolidate knowledge by building a simple temperature conversion function.
    - **Exercise:** Write a function that takes a temperature in Fahrenheit (`f64`) and returns the equivalent in Celsius (`f64`). Test it in `main`.
    - **Hint:** The formula is `(F - 32) * 5.0 / 9.0`.
    - **Book:** Chapter 3 (Project)

---

### **Phase 2: The Ownership System (Days 18-37)**

- ✅ **Day 18: Stack vs. Heap (Conceptual)**
    - **Topic:** Understanding the difference between stack and heap memory in Rust's context.
    - **Exercise:** No code today. Read the book chapter and draw a diagram showing how a simple variable is pushed onto the stack.
    - **Hint:** The stack is fast, ordered, and for data with a known, fixed size.
    - **Book:** Chapter 4.1
- ✅ **Day 19: The `String` Type**
    - **Topic:** Introducing the heap-allocated, growable `String` type.
    - **Exercise:** Create a mutable `String` and use `push_str()` to add text to it. Print the result.
    - **Hint:** `String::from("initial text")` is how you create one.
    - **Book:** Chapter 4.1
- ✅ **Day 20: Ownership and Move**
    - **Topic:** Observing how ownership is "moved" when a heap-allocated value is assigned to another variable.
    - **Exercise:** Assign `let s1 = String::from("hello");`. Then `let s2 = s1;`. Try to print `s1`. Observe the compiler error.
    - **Hint:** The error "borrow of moved value" is the key. Rust is preventing a double-free bug!
    - **Book:** Chapter 4.1
- ✅ **Day 21: The `clone()` Method**
    - **Topic:** Explicitly duplicating heap data using the `clone()` method to create a deep copy.
    - **Exercise:** Fix the code from Day 20 by using `let s2 = s1.clone();`. Now both `s1` and `s2` should be printable.
    - **Hint:** Cloning can be expensive, so use it thoughtfully.
    - **Book:** Chapter 4.1
- ✅ **Day 22: Stack-Only Data: Copy**
    - **Topic:** Understanding why simple scalar types don't "move" but are "copied" because they implement the `Copy` trait.
    - **Exercise:** Do the same exercise as Day 20, but with an `i32` (`let x = 5; let y = x;`). Observe that you can still use `x` afterwards.
    - **Hint:** Types on the stack have a known size and are cheap to copy.
    - **Book:** Chapter 4.1
- ✅ **Day 23: Ownership and Functions**
    - **Topic:** Seeing how passing a value to a function moves ownership into that function.
    - **Exercise:** Write a function `takes_ownership(some_string: String)`. In `main`, create a `String`, pass it to the function, and then try to use the string again in `main`.
    - **Hint:** The variable's ownership moves into the function's parameter, and is dropped when the function ends.
    - **Book:** Chapter 4.1
- ✅ **Day 24: Functions and Return Values**
    - **Topic:** Transferring ownership *out* of a function by returning a value.
    - **Exercise:** Write a function `gives_ownership() -> String` that creates a `String` and returns it. In `main`, call the function and store the result in a variable.
    - **Hint:** Ownership can be moved multiple times.
    - **Book:** Chapter 4.1
- ✅ **Day 25: The "Pass and Return" Idiom**
    - **Topic:** Taking ownership in a function and then returning it along with a result, a common but verbose pattern.
    - **Exercise:** Write a function `calculate_length(s: String) -> (String, usize)` that takes a string, returns a tuple of the string and its length.
    - **Hint:** This pattern is a bit clumsy. There must be a better way...
    - **Book:** Chapter 4.1
- ✅ **Day 26: Immutable References (`&`)**
    - **Topic:** "Borrowing" a value by creating a reference, which lets you use a value without taking ownership.
    - **Exercise:** Re-write `calculate_length` from Day 25 to take `s: &String` instead of `s: String`. In `main`, pass a reference (`&my_string`) and see that you can still use `my_string` after the call.
    - **Hint:** This is the "better way"! It's like a library book: you borrow it, but you don't own it.
    - **Book:** Chapter 4.2
- ✅ **Day 27: The Dereference Operator (`*`)**
    - **Topic:** Using the dereference operator `*` to get the value a reference is pointing to.
    - **Exercise:** Create a reference `r` to an `i32` variable `x`. Use `if *r == 5` to check the value.
    - **Hint:** In many cases, Rust dereferences automatically (like with the `.` operator), so you won't use `*` as often as in C.
    - **Book:** Chapter 15.1 (A little ahead, but relevant here)
- ✅ **Day 28: Mutable References (`&mut`)**
    - **Topic:** Creating a mutable reference to borrow a value and change it.
    - **Exercise:** Write a function `change(some_string: &mut String)` that appends ", world" to a string. Call it from `main` with a mutable reference.
    - **Hint:** Both the variable and the reference must be marked `mut`.
    - **Book:** Chapter 4.2
- ✅ **Day 29: The "One Mutable Reference" Rule**
    - **Topic:** Understanding Rust's key rule for preventing data races: you can have either one mutable reference OR any number of immutable references, but not both at the same time.
    - **Exercise:** In a new scope (`{}`), create a mutable reference to a `String`. Inside that scope, try to create a second mutable reference and observe the compiler error.
    - **Hint:** The scopes of references are what matters.
    - **Book:** Chapter 4.2
- ✅ **Day 30: The "Mutable vs. Immutable" Rule**
    - **Topic:** Exploring the other side of the borrowing rule: you can't have a mutable reference while immutable ones exist.
    - **Exercise:** Create two immutable references to a `String`. Then, try to create a mutable reference and observe the error.
    - **Hint:** Readers don't expect the data to suddenly change while they're reading it!
    - **Book:** Chapter 4.2
- ✅ **Day 31: Dangling References**
    - **Topic:** Seeing how the Rust compiler's borrow checker prevents dangling references.
    - **Exercise:** Write a function that tries to return a reference to a `String` created *inside* that function. The compiler will stop you with a lifetime error.
    - **Hint:** Compare this to C, where returning a pointer to a local variable is a classic bug.
    - **Book:** Chapter 4.2
- ✅ **Day 32: Review: Ownership and Borrowing Rules**
    - **Topic:** Consolidating the core rules of ownership.
    - **Exercise:** Write a short summary of the rules in your own words: 1. Each value has an owner. 2. Only one owner at a time. 3. When the owner goes out of scope, the value is dropped.
    - **Hint:** These rules are the foundation of Rust's safety.
    - **Book:** Chapter 4.2
- ✅ **Day 33: Introduction to Slices**
    - **Topic:** Understanding that a slice lets you reference a contiguous sequence of elements in a collection rather than the whole collection.
    - **Exercise:** No code. Read the book's motivating example for slices (the `first_word` function).
    - **Hint:** Slices provide a view into data without copying it.
    - **Book:** Chapter 4.3
- ✅ **Day 34: String Slices (`&str`)**
    - **Topic:** Using string slices to reference a part of a `String`.
    - **Exercise:** Create a `String` "hello world". Create slices for "hello" and "world".
    - **Hint:** Use range syntax: `&s[0..5]`.
    - **Book:** Chapter 4.3
- ✅ **Day 35: String Slices as Parameters**
    - **Topic:** Improving API flexibility by accepting `&str` instead of `&String` in functions.
    - **Exercise:** Write a function `first_word(s: &str) -> &str` that finds the first word in a string slice. Test it with both a `String` reference (`&my_string`) and a string literal (`"a string literal"`).
    - **Hint:** String literals are of type `&str`.
    - **Book:** Chapter 4.3
- ✅ **Day 36: Other Slices**
    - **Topic:** Applying the slice concept to other collections like arrays.
    - **Exercise:** Create an array of integers. Create a slice that contains a portion of the array's elements and print its length.
    - **Hint:** The syntax is the same: `&a[1..3]`.
    - **Book:** Chapter 4.3
- ✅ **Day 37: Review Phase 2**
    - **Topic:** Consolidating the concepts of Ownership, Borrowing, and Slices.
    - **Exercise:** Write a function that takes a slice of `i32`s and returns the sum. Add comments explaining why you are using a slice `&[i32]` as the parameter.
    - **Hint:** Using a slice is more general than using `&Vec<i32>`.
    - **Book:** Chapter 4

---

### **Phase 3: Custom Data Types (Days 38-50)**

- ✅ **Day 38: Defining `struct`s**
    - **Topic:** Grouping related data into a custom named type.
    - **Exercise:** Define a `struct Rectangle` with `width` and `height` fields, both of type `u32`.
    - **Hint:** This is similar to C structs, but with no trailing semicolon.
    - **Book:** Chapter 5.1
- ✅ **Day 39: Instantiating `struct`s**
    - **Topic:** Creating an instance of a struct and accessing its fields with dot notation.
    - **Exercise:** Create an instance of your `Rectangle` struct and calculate its area.
    - **Hint:** Use `let rect1 = Rectangle { width: 30, height: 50 };`.
    - **Book:** Chapter 5.1
- ✅ **Day 40: Tuple Structs**
    - **Topic:** Defining structs that behave like tuples but have a distinct type name.
    - **Exercise:** Define and create instances of `Color(u8, u8, u8)` and `Point(i32, i32, i32)` tuple structs.
    - **Hint:** These are useful when the field names would be redundant, e.g., `Point { x: i32, y: i32 }`.
    - **Book:** Chapter 5.1
- ✅ **Day 41: Structs and the `#[derive(Debug)]` Annotation**
    - **Topic:** Using the `Debug` trait to enable printing a struct for debugging purposes.
    - **Exercise:** Add `#[derive(Debug)]` above your `Rectangle` struct definition. In `main`, print an instance using the `{:?}` and `{:#?}` format specifiers.
    - **Hint:** The compiler error will guide you to add `#[derive(Debug)]`.
    - **Book:** Chapter 5.2
- ✅ **Day 42: Defining Methods with `impl`**
    - **Topic:** Associating functions with a specific struct type using an `impl` (implementation) block.
    - **Exercise:** Move the area calculation logic into a method `area(&self)` within an `impl Rectangle` block.
    - **Hint:** `&self` is shorthand for `self: &Self`, where `Self` is the type the `impl` block is for. It's a borrowed reference to the instance.
    - **Book:** Chapter 5.3
- ✅ **Day 43: Methods with More Parameters**
    - **Topic:** Creating methods that take parameters in addition to `&self`.
    - **Exercise:** Create a method `can_hold(&self, other: &Rectangle) -> bool` that checks if a second rectangle can fit inside the first.
    - **Hint:** The first parameter is still `&self`, followed by other parameters.
    - **Book:** Chapter 5.3
- ✅ **Day 44: Associated Functions (e.g., `::new`)**
    - **Topic:** Defining functions within an `impl` block that don't take `self` as a parameter, often used as constructors.
    - **Exercise:** Create an associated function `Rectangle::square(size: u32) -> Rectangle` that creates a square `Rectangle`.
    - **Hint:** You call these with `::` syntax, like `String::from`.
    - **Book:** Chapter 5.3
- ✅ **Day 45: Defining `enum`s**
    - **Topic:** Creating a custom type that can be one of a few different variants.
    - **Exercise:** Define an `enum IpAddrKind` with variants `V4` and `V6`. Create instances of both.
    - **Hint:** Rust enums are far more powerful than C enums; they are "sum types" or "tagged unions".
    - **Book:** Chapter 6.1
- ✅ **Day 46: Enums with Data**
    - **Topic:** Storing data directly inside an enum variant.
    - **Exercise:** Redefine the IP address enum to `enum IpAddr { V4(u8, u8, u8, u8), V6(String) }` and create instances.
    - **Hint:** Each variant can have different types and amounts of associated data.
    - **Book:** Chapter 6.1
- ✅ **Day 47: The `Option` Enum**
    - **Topic:** Understanding Rust's primary tool for handling nullability: `Option<T>`, with variants `Some(T)` and `None`.
    - **Exercise:** Write a function that takes an `Option<i32>` and tries to add one to the inner value.
    - **Hint:** `Option` is so important it's brought into scope by default. There is no `NULL` in safe Rust.
    - **Book:** Chapter 6.1
- ✅ **Day 48: The `match` Control Flow Operator**
    - **Topic:** Using `match` to compare a value against a series of patterns and execute code based on which pattern matches.
    - **Exercise:** Write a function `value_in_cents(coin: Coin) -> u8` for a `Coin` enum (`Penny`, `Nickel`, `Dime`, `Quarter`) that returns its worth using a `match` expression.
    - **Hint:** `match` arms must cover all possibilities. The compiler will enforce this.
    - **Book:** Chapter 6.2
- ✅ **Day 49: Matching on `Option<T>`**
    - **Topic:** Using `match` to handle the `Some` and `None` variants of an `Option`.
    - **Exercise:** Write a function `plus_one(x: Option<i32>) -> Option<i32>` that uses `match` to either increment the value inside `Some` or return `None`.
    - **Hint:** This is a very common pattern for safely handling optional values.
    - **Book:** Chapter 6.3
- ✅ **Day 50: `if let` for Concise Control Flow**
    - **Topic:** Using the `if let` syntax as a less verbose alternative to `match` when you only care about one specific pattern.
    - **Exercise:** Re-write a simple `match` statement on an `Option` to use `if let Some(value) = my_option { ... }` instead.
    - **Hint:** Use `if let` when you want to do something with one variant and ignore all others.
    - **Book:** Chapter 6.3

---

### **Phase 4: Organizing and Collecting Data (Days 51-65)**

- ✅ **Day 51: Introduction to Modules**
    - **Topic:** Using the `mod` keyword to group related code into separate namespaces within a single file.
    - **Exercise:** Create a `front_of_house` module with a child `hosting` module inside it. Define an `add_to_waitlist` function inside `hosting`.
    - **Hint:** Think of modules as a tree structure, with the crate root at the top.
    - **Book:** Chapter 7.1 & 7.2
- ✅ **Day 52: Paths and the `use` Keyword**
    - **Topic:** Bringing module paths into scope with the `use` keyword to avoid writing long paths repeatedly.
    - **Exercise:** From `main`, call the `add_to_waitlist` function from Day 51 using its full path. Then, add a `use` statement to bring `front_of_house::hosting` into scope and call the function with the shorter path.
    - **Hint:** `use crate::front_of_house::hosting;`
    - **Book:** Chapter 7.4
- ✅ **Day 53: The `pub` Keyword and `super`**
    - **Topic:** Making functions and modules public with `pub` to allow them to be called from outside their module, and using `super` to access parent modules.
    - **Exercise:** Make your `hosting` module and `add_to_waitlist` function public. Create a function at the root that uses the public path to call it.
    - **Hint:** By default, everything in Rust is private to its module.
    - **Book:** Chapter 7.3
- ✅ **Day 54: Separating Modules into Different Files**
    - **Topic:** Splitting a large module into its own file to better organize your project.
    - **Exercise:** Move the `front_of_house` module into its own file, `src/front_of_house.rs`, and declare it in `src/main.rs` with `mod front_of_house;`.
    - **Hint:** The file and folder structure follows the module tree.
    - **Book:** Chapter 7.5
- ✅ **Day 55: Review: The Module System**
    - **Topic:** Consolidating your understanding of modules, paths, `use`, and `pub`.
    - **Exercise:** Create a small library-like structure with a `utils` module in its own file containing a public function, and call that function from `main`.
    - **Hint:** A well-organized module structure makes code easier to navigate.
    - **Book:** Chapter 7
- ✅ **Day 56: Storing Lists with Vectors (`Vec<T>`)**
    - **Topic:** Using the growable, heap-allocated `Vec<T>` type to store a list of values of the same type.
    - **Exercise:** Create a `Vec<i32>`, add some numbers to it using `push`, and print the whole vector using `{:?}`.
    - **Hint:** `Vec` is Rust's version of a dynamic array or list.
    - **Book:** Chapter 8.1
- ✅ **Day 57: Reading and Iterating Over Vector Elements**
    - **Topic:** Accessing elements in a vector by index and iterating over its elements with a `for` loop.
    - **Exercise:** Create a `Vec<i32>`. Access the third element using `&v[2]`. Then, write a `for` loop that iterates over immutable references (`for i in &v`) and prints each element.
    - **Hint:** Accessing an index that's out of bounds will cause a panic!
    - **Book:** Chapter 8.1
- ✅ **Day 58: Using an Enum to Store Multiple Types**
    - **Topic:** A classic Rust pattern: using a vector of `enum`s to store different types of data in the same collection.
    - **Exercise:** Define an `enum SpreadsheetCell { Int(i32), Float(f64), Text(String) }`. Create a `Vec` that holds several variants of this enum.
    - **Hint:** The `enum` itself becomes the single type that the vector holds.
    - **Book:** Chapter 8.1
- ✅ **Day 59: `String` vs. `&str` Recap**
    - **Topic:** A deeper look at the heap-allocated, owned `String` and the borrowed, view-only `&str` (string slice).
    - **Exercise:** Create a `String`. Write a function that takes a `&str` and prints it. Pass both your `String` (as `&my_string`) and a string literal to the function.
    - **Hint:** This is a crucial distinction for writing flexible Rust code.
    - **Book:** Chapter 8.2
- ✅ **Day 60: Storing Key-Value Pairs with `HashMap<K, V>`**
    - **Topic:** Using the `HashMap` collection to store data associated with a key.
    - **Exercise:** Create a `HashMap` to store the scores of two teams. Insert the scores and print one of them by accessing its key.
    - **Hint:** You must first `use std::collections::HashMap;`.
    - **Book:** Chapter 8.3
- ✅ **Day 61: Hash Maps and Ownership**
    - **Topic:** Understanding how hash maps take ownership of their keys and values.
    - **Exercise:** Create a `String` for a key and a `String` for a value. Insert them into a hash map. Try to use the key and value variables after they have been moved into the map and observe the compiler error.
    - **Hint:** For types that implement `Copy` (like `i32`), the values are copied, not moved.
    - **Book:** Chapter 8.3
- ✅ **Day 62: Updating Values in a Hash Map**
    - **Topic:** Overwriting values, and only inserting a value if the key has no value yet using the `entry` API.
    - **Exercise:** Use the `entry` and `or_insert` methods to implement a simple counter. Iterate over a list of words, and use the hash map to track how many times you've seen each word.
    - **Hint:** `scores.entry(key).or_insert(value)` is the idiomatic way to do this.
    - **Book:** Chapter 8.3
- ✅ **Day 63: Project: Median and Mode**
    - **Topic:** Applying vector and hash map knowledge.
    - **Exercise:** Given a list of integers, use a vector and a hash map to return the median (middle value when sorted) and mode (most frequent value).
    - **Hint:** You'll need to sort the vector to find the median.
    - **Book:** Chapter 8.3 (Project)
- ✅ **Day 64: Project: Pig Latin**
    - **Topic:** Applying string manipulation knowledge.
    - **Exercise:** Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and "ay" is added, so "first" becomes "irst-fay". Words that start with a vowel have "hay" added to the end ("apple" becomes "apple-hay").
    - **Hint:** Remember that `&str` cannot be easily indexed by character. You may want to iterate over `chars()`.
    - **Book:** Chapter 8.3 (Project)
- ✅ **Day 65: Review Phase 4**
    - **Topic:** Consolidating knowledge of modules and standard collections.
    - **Exercise:** Create a small "employee database" program. It should use a hash map to store employees by department. Use modules to separate the database logic from the main executable logic.
    - **Hint:** `HashMap<String, Vec<String>>` could be a good data structure for this.
    - **Book:** Chapters 7 & 8

---

### **Phase 5: Robust Error Handling (Days 66-73)**

- ✅ **Day 66: Unrecoverable Errors with `panic!`**
    - **Topic:** Understanding `panic!`, which stops program execution immediately.
    - **Exercise:** Write a simple program and call `panic!("crash and burn");`. Observe the output.
    - **Hint:** `panic!` is for unrecoverable errors, like bugs that should not happen. It's the equivalent of a C program aborting.
    - **Book:** Chapter 9.1
- ✅ **Day 67: Recoverable Errors with `Result<T, E>`**
    - **Topic:** Using the `Result` enum, with variants `Ok(T)` and `Err(E)`, to handle errors that might be expected.
    - **Exercise:** Use `std::fs::File::open` to open a file. The return type is a `Result`.
    - **Hint:** Compare this to C's `fopen`, which returns `NULL` on error. `Result` is explicit about the success and error types.
    - **Book:** Chapter 9.2
- ✅ **Day 68: Matching on `Result<T, E>`**
    - **Topic:** Using a `match` statement to handle both the `Ok` and `Err` variants of a `Result`.
    - **Exercise:** Take the `Result` from Day 67 and `match` on it. If `Ok`, print "File opened." If `Err`, print "Failed to open file."
    - **Hint:** This is the foundational way to handle a `Result`.
    - **Book:** Chapter 9.2
- ✅ **Day 69: The `?` Operator for Propagating Errors**
    - **Topic:** Using the `?` operator as a shortcut to return the `Err` part from a function, or unwrap the `Ok` part if there is no error.
    - **Exercise:** Write a function that reads a username from a file. Use `File::open("hello.txt")?` inside the function.
    - **Hint:** The `?` operator can only be used in functions that return a `Result` or `Option`.
    - **Book:** Chapter 9.2
- ✅ **Day 70: Chaining `?` Operator Calls**
    - **Topic:** Becoming comfortable with chaining multiple fallible operations together using `?`.
    - **Exercise:** Extend the function from Day 69. After opening the file with `?`, read its contents into a string, also using `?`.
    - **Hint:** `std::fs::read_to_string` provides a more direct way to do this.
    - **Book:** Chapter 9.2
- ✅ **Day 71: `main` can return `Result`**
    - **Topic:** Simplifying error handling in your main function by allowing `main` to return a `Result`.
    - **Exercise:** Change your `main` function signature to `fn main() -> Result<(), Box<dyn std::error::Error>>`. Now you can use the `?` operator directly inside `main`.
    - **Hint:** `Box<dyn Error>` is a "trait object," which we'll cover later. For now, it means "any kind of error."
    - **Book:** Chapter 9.2
- ✅ **Day 72: When to `panic!` vs. Return `Result`**
    - **Topic:** Developing an intuition for when an error is a bug (`panic!`) versus an expected outcome (`Result`).
    - **Exercise:** No code. Write down a few scenarios. Example: A function is passed a `null` pointer from C. (Should probably `panic!`). Example: A file might not exist at a path the user gave. (Should return `Result`).
    - **Hint:** If the code can't possibly recover and continue in a meaningful way, `panic!` might be appropriate.
    - **Book:** Chapter 9.3
- ✅ **Day 73: Review Phase 5**
    - **Topic:** Consolidating error handling concepts.
    - **Exercise:** Write a program that takes a filename from a command-line argument, opens it, and prints its contents. Use `?` and a `Result`-returning `main` function to handle all potential errors gracefully.
    - **Hint:** Combine knowledge from argument parsing and file I/O with the `?` operator.
    - **Book:** Chapter 9

---

### **Phase 6: Abstraction and Generics (Days 74-90)**

- ✅ **Day 74: Removing Duplication with Generic Functions**
    - **Topic:** Using generic type parameters to write a function that can operate on values of multiple types.
    - **Exercise:** Write a generic function `largest<T>(list: &[T]) -> T` that can find the largest element in a slice of any type that supports ordering. You will get a compiler error.
    - **Hint:** The compiler error will tell you that it doesn't know how to compare the generic type `T`. This leads into traits.
    - **Book:** Chapter 10.1
- ✅ **Day 75: Generics in Struct Definitions**
    - **Topic:** Defining a struct with one or more generic type parameters.
    - **Exercise:** Define a `struct Point<T> { x: T, y: T }`. Create instances of `Point` with both integer and float values.
    - **Hint:** This allows you to have, for example, `Point<i32>` and `Point<f64>` from the same definition.
    - **Book:** Chapter 10.1
- ✅ **Day 76: Introduction to Traits**
    - **Topic:** Defining shared behavior with a `trait`, which is similar to an interface in other languages.
    - **Exercise:** Define a `trait Summary` with a method signature `fn summarize(&self) -> String;`.
    - **Hint:** A trait defines a set of methods a type must implement to claim it has that behavior.
    - **Book:** Chapter 10.2
- ✅ **Day 77: Implementing a Trait on a Type**
    - **Topic:** Using an `impl` block to provide the concrete implementation of a trait's methods for a specific type.
    - **Exercise:** Implement the `Summary` trait for a `NewsArticle` struct and a `Tweet` struct.
    - **Hint:** The implementation for each struct will be different, but they will both satisfy the `Summary` trait.
    - **Book:** Chapter 10.2
- ✅ **Day 78: Trait Bounds**
    - **Topic:** Using trait bounds on a generic function to accept any type that implements a specific trait.
    - **Exercise:** Fix the `largest` function from Day 74 by adding a trait bound: `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T`.
    - **Hint:** The `T: PartialOrd` part tells the compiler that `T` is a type that can be ordered.
    - **Book:** Chapter 10.2
- ✅ **Day 79: `impl Trait` as a Parameter Type**
    - **Topic:** Using `impl Trait` syntax for a simpler way to specify a parameter is a generic type that implements a particular trait.
    - **Exercise:** Write a function `notify(item: &impl Summary)` that takes any type implementing the `Summary` trait.
    - **Hint:** This is syntactic sugar for the longer trait bound syntax.
    - **Book:** Chapter 10.2
- ✅ **Day 80: Introduction to Lifetimes**
    - **Topic:** Conceptual overview of lifetimes, which are how the borrow checker ensures all borrowed references are valid.
    - **Exercise:** Re-read the dangling reference example from Day 31. Try to write a function `longest(x: &str, y: &str) -> &str`. Observe the compiler error.
    - **Hint:** The compiler doesn't know if the returned reference will refer to `x` or `y`, so it can't determine if it will be valid.
    - **Book:** Chapter 10.3
- ✅ **Day 81: Lifetime Annotation Syntax**
    - **Topic:** Learning the syntax for lifetime annotations, e.g., `&'a str`.
    - **Exercise:** Fix the `longest` function from Day 80 by adding lifetime parameters: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`.
    - **Hint:** This tells the compiler that all the references in the function signature must have the same lifetime `'a`.
    - **Book:** Chapter 10.3
- ✅ **Day 82: Lifetime Annotations in Struct Definitions**
    - **Topic:** Using lifetime annotations when a struct needs to hold a reference.
    - **Exercise:** Define a struct `ImportantExcerpt<'a>` that holds a `part: &'a str`.
    - **Hint:** This tells the compiler that an instance of `ImportantExcerpt` cannot outlive the reference it holds.
    - **Book:** Chapter 10.3
- ✅ **Day 83: The Static Lifetime (`'static`)**
    - **Topic:** Understanding the special `'static` lifetime, which means the reference can live for the entire duration of the program.
    - **Exercise:** Create a variable `s` with a `&'static str` type. A string literal is a good example.
    - **Hint:** All string literals have the `'static` lifetime.
    - **Book:** Chapter 10.3
- ✅ **Day 84: Accepting Command-Line Arguments & Reading a File**
    - **Topic:** Parsing arguments from the command line and reading file contents. This day covers the initial setup of the program's inputs.
    - **Exercise:** Write a program that takes two command-line arguments: a path to a file and a string to search for. Read the file specified in the first argument and then print its contents to the screen. For now, it's okay to use `.expect()` for basic error handling if the file can't be read.
    - **Hint:** Use `std::env::args().collect()` to get the arguments as a `Vec<String>`. Use `std::fs::read_to_string()` to read the file's contents into a string.
    - **Book:** Chapter 12.1 & 12.2
- ✅ **Day 85: Refactoring for Modularity**
    - **Topic:** Separating configuration and logic from the `main` function to improve the program's structure and clarity.
    - **Exercise:** Create a `Config` struct to hold the `query` and `filename` values. Implement an associated function `Config::new` that takes the vector of arguments and returns a `Config` instance. Move the core logic of reading the file and printing its contents into a separate `run` function that takes the `Config` instance as an argument.
    - **Hint:** The `main` function's primary role should be to call `Config::new` and then pass the result to `run`. This makes the logic in `main` easier to read.
    - **Book:** Chapter 12.3 (up to "Returning a `Result` from `new`")
- ✅ **Day 86: Improving Error Handling**
    - **Topic:** Replacing `panic!` with a more robust `Result` type for handling potential errors in argument parsing and file operations.
    - **Exercise:** Modify `Config::new` to return a `Result<Config, &'static str>` instead of panicking if there are too few arguments. Change the `run` function to return a `Result<(), Box<dyn Error>>` to handle I/O errors. Update `main` to handle these `Result` types gracefully without panicking.
    - **Hint:** In `main`, use an `if let Err(e)` block to check for and handle the error returned from `run`.
    - **Book:** Chapter 12.3 (finishing the section)
- ✅ **Day 87: Splitting Code into a Library Crate**
    - **Topic:** Organizing the project by separating the reusable logic into a library crate, leaving `main.rs` as the user-facing binary.
    - **Exercise:** Create `src/lib.rs` and move your `run` function, `Config` struct, and its `new` function into it. Make them public using the `pub` keyword. Modify `src/main.rs` to import and use the library crate's public API.
    - **Hint:** Your `main.rs` will get much shorter. It will only need to parse arguments into a `Config` object from your library and call the `run` function from your library.
    - **Book:** Chapter 12.4 (up to "Writing a Failing Test")
- ✅ **Day 88: Test-Driven Development for Search Logic**
    - **Topic:** Using Test-Driven Development (TDD) to implement the core search functionality in the library crate.
    - **Exercise:** In `src/lib.rs`, write a test for a new `search` function that fails initially. Then, implement the `search` function with the signature `pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>`. The function should find and return all lines in `contents` that contain the `query`. Finally, integrate the `search` function into `run`.
    - **Hint:** The lifetime `'a` is necessary to tell Rust that the returned vector of string slices is tied to the lifetime of the `contents` slice.
    - **Book:** Chapter 12.4 (finishing the section)
- ✅ **Day 89: Adding Case-Insensitivity with Environment Variables**
    - **Topic:** Making the program's behavior configurable at runtime using environment variables.
    - **Exercise:** Implement a new `search_case_insensitive` function and write a test for it. Add a new boolean field to your `Config` struct to control case sensitivity. Set this field's value based on whether an environment variable (e.g., `IGNORE_CASE`) is set. Modify `run` to call the appropriate search function based on this configuration.
    - **Hint:** Use `std::env::var("YOUR_VAR_NAME").is_ok()` to check if an environment variable exists. The `to_lowercase()` method will be helpful for the case-insensitive search.
    - **Book:** Chapter 12.5
- ✅ **Day 90: Writing Error Messages to Standard Error (`stderr`)**
    - **Topic:** Directing error messages to `stderr` to follow command-line program conventions, allowing users to redirect successful output (`stdout`) without including errors.
    - **Exercise:** Refactor the error handling in `main.rs`. Instead of printing errors with `println!`, use the `eprintln!` macro. Change `main` to return a `Result<(), Box<dyn Error>>` so you can use the `?` operator for more concise error handling.
    - **Hint:** By having `main` return a `Result`, you can simplify the error handling logic significantly. Rust will automatically handle printing the error from an `Err` variant to `stderr` when `main` exits.
    - **Book:** Chapter 12.6

---

### **Phase 7: Advanced Features & Beyond (Days 91-99)**

- ✅ **Day 91: Closures (Anonymous Functions)**
    - **Topic:** Using closures, which are function-like constructs you can store in a variable.
    - **Exercise:** Define a closure that takes one number and adds one to it. Store it in a variable and call it.
    - **Hint:** `let plus_one = |x| x + 1;`
    - **Book:** Chapter 13.1
- ✅ **Day 92: Capturing the Environment with Closures**
    - **Topic:** Understanding how closures can capture values from the scope in which they are defined.
    - **Exercise:** Create a variable `x`. Define a closure that uses `x`. Call the closure and print the result.
    - **Hint:** Closures can capture by reference, mutable reference, or by value.
    - **Book:** Chapter 13.1
- ✅ **Day 93: Processing Data with Iterators**
    - **Topic:** Using the `Iterator` trait to perform tasks on a sequence of items.
    - **Exercise:** Create a vector. Call the `.iter()` method on it. Use a `for` loop to consume the iterator and print its values.
    - **Hint:** The `for` loop takes ownership of the iterator and calls `.next()` on it repeatedly.
    - **Book:** Chapter 13.2
- ✅ **Day 94: Iterator Adapters (`map`, `filter`)**
    - **Topic:** Using methods that transform iterators into different kinds of iterators (iterator adapters).
    - **Exercise:** Take a vector of numbers, call `.iter()`, then `.map()` with a closure to add 1 to each item, and then collect the results into a new vector.
    - **Hint:** Iterator adapters are lazy; they don't do any work until you call a consuming method.
    - **Book:** Chapter 13.2
- ✅ **Day 95: Consuming Iterators (`collect`)**
    - **Topic:** Using methods that consume an iterator and produce a final value, like `collect()`.
    - **Exercise:** Create a vector of numbers `v`. Call `v.iter().map(|x| x * 2).collect()` to create a new vector.
    - **Hint:** You may need to provide a type annotation: `let new_vec: Vec<_> = ...`
    - **Book:** Chapter 13.2
- ✅ **Day 96: Automated Tests**
    - **Topic:** Writing functions marked with `#[test]` that `cargo` can run to verify your code's correctness.
    - **Exercise:** Write a simple `add(a: i32, b: i32) -> i32` function. Create a `tests` module and write a test function to assert that `add(2, 2)` equals 4. Run `cargo test`.
    - **Hint:** `#[cfg(test)]` tells the compiler to only compile this code when running tests.
    - **Book:** Chapter 11
- ✅ **Day 97: Smart Pointers (`Box<T>`)**
    - **Topic:** Using `Box<T>`, the simplest smart pointer, to allocate values on the heap.
    - **Exercise:** Create a `Box` to store an `i32` on the heap.
    - **Hint:** This is the foundation for data structures like linked lists.
    - **Book:** Chapter 15.1
- ✅ **Day 98: Concurrency**
    - **Topic:** Spawning threads to run code simultaneously.
    - **Exercise:** Use `std::thread::spawn` to create a new thread that prints a message. Use a handle to wait for the thread to finish.
    - **Hint:** Use `move` closures to transfer ownership of data to a new thread.
    - **Book:** Chapter 16
*   **Day 99: Final Review & Next Steps**
    *   **Topic:** Reflecting on the journey from C to Rust and planning what to learn next.
    *   **Exercise:** Go back to your `minigrep` project. Can you refactor it using iterators? Can you improve the error handling? Write down what you found most challenging about Rust and what you found most rewarding.
    *   **Hint:** Consider exploring popular libraries like `serde` for serialization or `tokio` for asynchronous programming as your next step.
    *   **Book:** Chapter 20 (Final Project: A Multi-Threaded Web Server)
