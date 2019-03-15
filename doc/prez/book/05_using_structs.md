---
slug: 05-using-structs
title: Using structs
page: True
menu: [book]
---
# Defining & Instantiating Structs

* Similar to tuples, but you name each piece of data. Definition:

><pre>
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
</pre>

* Instanciate:

><pre>
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
</pre>

* Access / Modify
* Entire instance must be mutable

><pre>
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
>
>user1.email = String::from("antoheremail@example.com");
</pre>

* Like any expression, we can construct an instance as last expression in a function --> this will return that new instance
* Can use special syntax if field has same name that parameter

><pre>
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
</pre>

* Creating instances from other instances : syntax __..__
The syntax __..__ specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance

><pre>
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
</pre>

* Using tuple stucts without named fields to create different types

><pre>
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
>
>let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
</pre>

* Unit-like structs: without any field  
Have similarity to _()_, the unit type  
Can be useful if you need to implement a trait on some type but don't have any data that you want to store in the type itself (see §10)

* Ownership of struct data

It's possible for structs to store references to data owned by something else, but to do so requires the use of _lifetimes_ (see §10).

# Example program using Structs

* See example rs file

* How to print debugging content of struct

Using annotation:

><pre>#[derive(Debug)]  
struct Rectangle {
    width: u32,
    height: u32,
}
>
>fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
>
>    println!("rect1 is {:?}", rect1);
}
</pre>

* Use __{:?}__ to display on 1 line, or __{:#?}__ to display on multiple lines:

><pre>
rect1 is Rectangle {
    width: 30,
    height: 50
}
</pre>

* See Appendic C for traits and their behaviors
* See §10 for custom behavior, and how to create own traits

# Method syntax

* Using __impl__

><pre>#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
</pre>

* Multiple parameters

><pre>
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
</pre>

* Associated functions

If a function don't take self --> Equivalent of Class method, not instance method.
Can be used as constructor

><pre>
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
</pre>

* Multiple __impl__ blocks
It's valid to have multiple __impl__ blocks. No reason to separate, but can be useful (see §10 : generic types & traits)
