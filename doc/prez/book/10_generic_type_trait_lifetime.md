---
slug: 10-generic-type-trait-lifetime
title: Generic type, trait and lifetime
page: True
menu: [book]
---


# Generic types

* Removing Duplication by Extracting a Function

Replace:

><pre>
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
</pre>

By:

Note this will not compile yet

><pre>
fn largest< T >(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
</pre>  

></pre>
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
</pre>

std::cmp::PartialOrd is a _trait_

* In struct definition

><pre>
struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
</pre>

* In enum definitions

><pre>
enum Result<T, E> {
    Ok(T),
    Err(E),
}
</pre>

* In method definition

><pre>
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
</pre>

You could implement methods only on Point<f32>

><pre>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
</pre>

* No performance cost using generics

# Trait

* equivalent to a feature often called _interfaces_ in other languages

* Defining a Trait

><pre>
pub trait Summary {
    fn summarize(&self) -> String;
}
</pre>

* Implementing a Trait on a Type

><pre>
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
</pre>

* Default Implementations

><pre>
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
</pre>

* Traits as arguments

><pre>
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
</pre>

* Traits Bounds

><pre>
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
</pre>

This is only a shorter way to write from:

><pre>
pub fn notify(item1: impl Summary, item2: impl Summary) {
</pre>

to:

><pre>
pub fn notify<T: Summary>(item1: T, item2: T) {
</pre>

* Specify multiple traits with _+_

><pre>
pub fn notify(item: impl Summary + Display) {
</pre>

or

><pre>
pub fn notify<T: Summary + Display>(item: T) {
</pre>

* _where_ clauses

Instead of:

><pre>
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
</pre>

Do:

><pre>
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
</pre>

* Returning Traits

><pre>
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
</pre>

This will work ony if you have a single type that you're returning. (See §17 "Using Trait Objects that Allow for Values of Different Types")

* Fixing the _largest_ function with Trait Bounds

><pre>
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
</pre>

* Using Trait Bounds to Conditionally Implement Methods

--> implement only when implements all traits

><pre>
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
</pre>

--> implement a trait for any type that implements another trait

><pre>
impl<T: Display> ToString for T {
    // --snip--
}
</pre>

# Validating References with Lifetimes

* Every reference has a _lifetime_, which is the scope for which that reference is valid
* We must annotate lifetimes when the lifetimes of references could be related in a few different ways
* Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid
* See more advances lifetimes in §19

* Preventing Dangling References with lifetimes

Bad:
><pre>
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
</pre>

OK:

><pre>
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}  
</pre>

* Generic lifetimes in functions

><pre>
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
</pre>

won't compile

><pre>
error[E0106]: missing lifetime specifier
= help: this function's return type contains a borrowed value, but the
signature does not say whether it is borrowed from `x` or `y`
</pre>

but we don't know either, because this is x or y, depending on the size

* Lifetime annotation syntax

Names of lifetime parameters must start with an apostrophe, and are useallly all lowercase and very short, like generic types. most people use _'a_. lifetime parameter annotations placed after the _&_ of a reference, using a space to separate the annotation from the reference's type

><pre>
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
</pre>

* Lifetime annotation in function signatures

The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime. We’ll name the lifetime 'a and then add it to each reference

><pre>
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
</pre>

* Lifetime annotation in Struct definitions

It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.

><pre>
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
</pre>

* Lifetime Elision

rules to apply (for simplier code syntax). If there are still some param without lifetime, so explicit lifetime is needed

    * rule1, each param that is a reference gets its own lifetime param

    * rule2, if there is exactly one input lifetime param, that lifetime is assigned to all output lifetime param

    * rule3, if there are multiple input lifetime param, but one of them is &self or &mut self because this is a method, the lifetime of seld is assigned to all output lifetime param

* Lifetime annotation in method definitions

><pre>
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
</pre>

><pre>
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
</pre>

* static lifetime

><pre>
let s: &'static str = "I have a static lifetime.";
</pre>

Which denotes the entire duration of the program. All string literals have the _'static_ lifetime

* Generic type parameters, trait bounds, and lifetimes together

><pre>
use std::fmt::Display;
fn longest_with_an_announcement< 'a, T > (x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
</pre>
