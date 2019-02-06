---
slug: 02-programming-a-guessing-game
title: Programming a guessing game
page: True
menu: [book]
---
# New project

> cargo new guessing_game  
> cd guessing_game

> cargo run

# Using standard library

> use std::io ;

# Variable declaration

> let mut guess = String::new() ;

By default a variable is immutable

# Using keyboard input

> io::stdin().read_line(&mut guess).expect("Failed to read line")

* By reference: __&__
* make it mutable using __&mut__

# Failure check

>.except("Failed to read line");

read_line returns a value of type __io::Result__. This is an enum (see §6)

* _Result_ variant are _Ok_ or _Err_:
    * _Ok_ variant --> operation was successful, and inside is the generated value
    * _Err_ variant --> operation failed, and inside is info about how / why operation failed

* _io::Result_ instance has an _expect_ method
    * if instance of _io::Result_ is _Err_, _expect_ will cause the program to crash, and display the message
    * if instance of _io::Result_ is _Ok_, _expect_ will take the return value that _Ok_ is holding

* If we don't call _expect_, the program will compile, but with warning

See §9 for recovering from errors

# Printing with placeholders

> println!("You guessed: {}", guess);  
> println!("x = {} and y = {}", x, y);

# Using Crate

* modify Cargo.toml to include _rand_ crate as dependencies

>[dependencies]  
>  
>rand = "0.3.14"

0.3.14 is shorthand for ^0.3.14

* Cargo.lock file

will contains version used when building

* Updating a create to get a new version

>cargo update  
Need to update _Cargo.toml_ file to new version

# Generate random number

>use rand::Rng;  
>  
>let secret_number = rand::thread_rng().gen_range(1, 101);  

Chapter §10 will talk about trait (_Rng_) in detail

# Comparing number

> use std::cmp::Ordering;  
>  
>fn main() {  
>  
>  match guess.cmp(&secret_number) {  
>     Ordering::Less => println!("Too small!"),  
>     Ordering::Greater => println!("Too big!"),  
>     Ordering::Equal => prinln!("You win!"),  
>  }
>
>}

Ordering is an enum, with variants:  

* Less
* Greater
* Equal

Will be covered in detail in chapter §6 and §18

# Type conversion

>let mut guess = String::new();  
>io::stdin().read_line(&mut guess).expect("Failed to read line");  
>let guess: u32 = guess.trim().parse().expect("Please type a number");  

* guess can be re-defined with another type: shadowing (see §3)
* trim() to delete new line CR
* parse method on strings parses a string into some kind of number

# Loop

>loop {  
>   //  
>}  

>break;  

To exit the loop

# Handling invalid input

>let guess: u32 = match guess.trim.parse() {  
>   Ok(num) => num,  
>   Err(\_) => continue,  
>}  

Using underscore to catchall
