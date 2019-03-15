---
slug: 06-enums
title: Enums
page: True
menu: [book]
---
# Defining an Enum

* definition

><pre>
enum IpAddrKind {
    V4,
    V6,
}
</pre>

* create instances

><pre>
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
</pre>

* Using as parameter

><pre>
fn route(ip_type: IpAddrKind) { }
route(IpAddrKind::V4);
route(IpAddrKind::V6);
</pre>

* Data inside Enum variant

><pre>
enum IpAddr {
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
</pre>

* Possibility to have different type inside each enum variant
(this type can be structs too)

><pre>
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
</pre>

* We can implement methods for Enums

><pre>
>enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));
m.call();
</pre>

# __Option__ Enum

* Defined in standard library

><pre>
enum Option< T > {
    Some(T),
    None,
}
</pre>

* Option< T > is defined in prelude, so don't need to bring it into scope explicitly, and variants _Some_ and _None_ can be used direclty without the _Option::_ prefix


* Using it

><pre>
et some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option < i32 > = None;
</pre>

# __Match__

* Syntax

><pre>
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
</pre>

* Patterns that bind to values

><pre>#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
</pre>

* Matching with _Option< T >_

><pre>
fn plus_one(x: Option< i32 >) -> Option< i32 > {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
</pre>

* Be exhaustive

    * We have to match all possible values of the Enum
    * Using "\_" placeholder for all remaining values

><pre>
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
</pre>

* __If let__

Replace this:

><pre>
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
</pre>

By this:

><pre>
if let Some(3) = some_u8_value {
    println!("three");
}
</pre>

We can use else too. Replace:

><pre>
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
</pre>

By:

><pre>
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
</pre>
