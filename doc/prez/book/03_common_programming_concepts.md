---
slug: 03-common-programming-concepts
title: Common Programming Concepts
page: True
menu: [book]
---

# Mutability

* By default, variables are immutables

* Define a variable immutable / mutable

><pre>
let x = 5 ;  
let mut y = 6 ;  
</pre>

# Constant

* defined with _const_
* type must be specified
* can be declared in any scope
* can only be set by a constant expression, can't be defined as result of function or with value defined only at runtime

> const MAX_POINTS: u32 = 10_000 ;

( _ can be used as visual separator)

# Shadowing

* Can shadow a variable by declaring a variable with same name

><pre>
fn main  {
  let x = 5 ;
  let x = x + 1 ;
}</pre>

* Not the same as using mut, because x is still immutable
* We can change the type of x with the new variable

# Data types

* Two data type subset: scalar and compound
* Define with type
>let guess: u32 = "42".parse().expect("Not a Number");

* Scalar: integers


| Length | Signed | Unsigned |
|:--:|:--:|:--:|
| 8-bits | i8 | u8 |
| 16-bits | i16 | u16 |
| 32-bits | i32 | u32 |
| 64-bits | i64 | u64 |
| arch | isize | usize |

<br/>

| Integer literal | Example |
|:--:|:--:|
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte (u8 only) | b'A' |

* Scalar: floating point numbers
    * f32
    * f64 (default)

* numeric operations
><pre>
fn main() {
  let sum = 5 + 10;
  let sub = 95.6 - 4.3;
  let prod = 4 * 30;
  let div = 56.7 / 32.2;
  let rem = 43 % 5;
}</pre>

* Scalar: boolean
><pre>
fn main() {
  let t = true;
  let f: bool = false; // with explicit type declaration
}</pre>

* Scalar: characters
Defined with simple quote, where strings defined with double quote

><pre>
fn main() {
  let c = 'z';
}</pre>

* Compound type: Tuple

><pre>
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup; // destructure the tuple
  let x = tup.1; // access to index of tuple, here y
}</pre>

* Compound type: Array
    * Every element must have same type
    * Useful when want data allocated on the stack rather than the heap (see later)
    * Fixed number of elements (see vector for changing nb of elements)

><pre>
fn main() {
  let a = [1, 2, 3, 4, 5]; // define
  let b = a[1]; // value = 2
  let element = a[10]; // will compile, but program runs with error
}</pre>

# Functions
* can be defined before or after they are used

><pre>
fn another_function() {
  println!("Another function");
}
</pre>

## Function parameters
* type must be defined

><pre>
fn main() {
  another_function(5);
}
fn another_function(x: i32, y: f64) {
  println!("Values are {} and {}", x, y);
}
</pre>

## Statement & Expression
* Statement: does not return a value
    * Example: let x = 5;  // does not return 5, but nothing
* Expression return a value

><pre>
fn main() {
  let y = {
    let x = 3 ;
    x + 1 // no ;
  };
}</pre>

## Function with return values

><pre>
fn five() -> i32 {
  5
}
fn main() {
  let x = five();
}</pre>

# Comments

* with //
* Documentation comments will be see later

# Control flow

## If / else if / else

><pre>
fn main() {
  let number = 6 ;
  if number % 4 == 0 {
    println!("Number is divisible by 4");
  } else if number % 3 == 0 {
    println!("Number is divisible by 3");
  } else if number % 2 == 0 {
    println!("Number is divisible by 2");
  } else {
    println!("Number is not divisible by 4, 3 or 2");
  }
}</pre>

Using if in let statement:  
must be same type for all conditions
><pre>
fn main() {
  let condition = true;
  let number = if condition {
    5
  } else {
    6
  };
}</pre>

## Loops

* infinite loops

><pre>
fn main() {
    loop {
      println!("loop");
      break;
  }
}</pre>

* conditional loops

><pre>
fn main() {
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number = number - 1;
  }
  println("Liftoff!");
}</pre>

* Looping through a collection

><pre>
fn main() {
  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("Value is: {}", element);
  }
}</pre>

><pre>
fn main() {
  for number in (1..4).rev() { // 1 is included, 4 is excluded
    println!("{}", number);
  }
  println!("Liftoff!");
}
