---
slug: 04-understanding-ownership
title: Understanding Ownership
page: True
menu: [book]
---
# Stack & Heap

## Stack

* stack is fast
* last in, first out
* all data have a known, fixed size

## Heap

* data with a size unknown at compile time or size that might change
* allocating on the heap: ask for some amount of space on memory
* slower than accessing data on the stack because you have to follow a pointer to get there


## Functions

* When calling a function, values passed into the function (including pointers to data on the heap) and function's local variables are pushed onto the stack.
* When the function is over, those values are popped off the stack


# Ownership

* Each value has a variable that's called its _owner_
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped

## Variable scope

* Where the variable is valid, similar to that in other programming languages

## String type as example

* String literal: we know the contents at compile time, so the text is hardcoded into the final executable
* String type is mutable, so:
    * Memory must be requested from the OS at runtime (done by _String::from_)
    * We need a way of returning this memory to the OS when we're done with our String. The memory is automatically returned once the variable that owns it goes out of scope. A function _drop_ is automatically vcalled

## Variables & Data: Move

><pre>
let x = 5;
let y = x;
</pre>

* This create 2 variables, both equal to 5. Integers are simple values, pushed onto the stack

><pre>
let s1 = String::from("hello");
let s2 = s1;
</pre>

* On the left : stored on the stack
* On the right: stored on the heap

<img src="../img/trpl04-02.svg" width="70%">

><pre>
let s1 = String::from("hello");
let s2 = s1;
</pre>

* Here, s1 goes out of scope, is no longer valid. s1 was moved into s2 (not a deep copy, more like shallow copy)

<img src="../img/trpl04-04.svg" width="70%">

* Rust never create automatically deep copies

## Variables & data interact: Clone

* If we want a deep copy (not only the stack data), use _clone_

><pre>
let s1 = String::from("hello");
let s2 = s1.clone();

## Stack only data: Copy

><pre>
let x = 5;
let y = x;
</pre>

* Here x is still valid, because integers are not on the heap, but on the stack
* Rust has a special annotation called the _Copy_ trait that we can place on types like integers that are stored on the stack (Cf trait on §10)
* If a type has the _Copy_ trait, an older variable is still usable after assignment
* Rust won't let us annotate a type with the _Copy_ trait if the type, or any of its parts, has implemented the _Drop_ trait.
* Learn about how to add the _Copy_ annotation to type on Appendix C.
* Following types aer _Copy_:
    * All the integer types, (example u32)
    * Boolean type _bool_, with values _true_ and _false_
    * all floating point types, such as _f64_
    * Character type _char_
    * Tuples, if they only contain types that are also _Copy_. Example _(i32, i32)_ is _Copy_, but _(i32, String)_ is not.


## OwnerShip and functions

* Works similar to assigning a value to a variable: will move or copy

><pre>
fn main() {
  let s = String::from("hello");
  take_ownership(s); // s value moves into the function, s is no longer valid
  let x = 5;
  makes_copy(x); // i32 is Copy, so it's OK to still use x after
} // x go out of scope here. s too. Because s value was moved, nothing special happens

> fn takes_ownership(some_string: String) {
  println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called. Memory is freed
>
> fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens
</pre>

## Return values and scope

* Returning values can also transfer ownership

><pre>
fn main() {
  let s1 = gives_ownership(); // give_ownership moves its return value into s1
  let s2 = String::from("hello"); // s2 comes into scope
  let s3 = takes_and_gives_back(s2); // s2 is moved into function, which alse moves its return value into s3
} // s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped

>fn gives_ownership() -> String { // function will move its return value into the function that calls it
  let some_string = String::from("hello");
  some_string // some_string is returned and moves out to the calling function
}

>fn takes_and_gives_back(a_string: String) -> String {
  a_string // is returned and moves out to the calling function
}
</pre>

* We can return tuple if we want to use inside the function, and reuse after. (But we will use references instead)

><pre>
fn main() {
  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
}

>fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}
</pre>

# References and Borrowing

* references : not mutable

><pre>
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
>
>fn calculate_length(s: &String) -> usize {
    s.len()
}
</pre>

* Mutable references
* Can have only one mutable reference in a particular scope
* Can not have mutable and immutable in same time. You can have, at any given given time, either:
    * One mutable reference, OR
    * Any number of immutable

><pre>
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}
>
>fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
</pre>

* Dangling references : Do not

><pre>fn main() {
    let reference_to_nothing = dangle();
}
>
>fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
</pre>

That returns a reference to not existing data, but do:

><pre>fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
</pre>

that moves ownership

# String Slices

* Reference a contiguous sequence of elements in a collection rather that the whole collection
* Slice does not have ownership
* String slice is a __reference__ to part of a String

><pre>let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
</pre>

* start..end : Start is included, end is excluded
* start..=end : Start is included, end is included too
* If start at 0, can omit it: _&s[..5]_;
* If go to the end, can omit it: _&[5..]_;
* that "string slice" is _&str_

><pre>fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
>     for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
>    &s[..]
}
</pre>

* Best practice: Using this kind of function:

><pre>
fn first_word(s: &str) -> &str {
</pre>

Because you can call it with &str and with String:

><pre>
let my_string = String::from("hello world");
let my_string_literal = "hello world";
let word = first_word(&my_string[..]);
let word = first_word(&my_string_literal[..]);
let word = first_word(my_string_literal);
</pre>

# Other slices

><pre>
let a = [1, 2, 3, 4, 5];
> let slice = &a[1..3];
</pre>
