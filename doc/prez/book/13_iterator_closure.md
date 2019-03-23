---
slug: 13-iterator-closure
title: Iterators and closures
page: True
menu: [book]
---
# Closures

* Anonymous functions that can capture their environment
* Can be save in a variable or pass as arguments to other functions
* You can create the closure in one place, and then call the closure to evaluate it in a different context
* Can capture values from the scope in which they're defined

## Creating an abstraction of behavior with closures

><pre>
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
</pre>

## Closure type inference and annotation

><pre>
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
</pre>

* Type of param must be the same for each call (and defined with the first call)

## Storing closures using generic parameters and the _Fn_ traits

* All closures implement at least one of the traits:
    * Fn
    * FnMut
    * FnOnce

><pre>
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
</pre>

* Cache system

><pre>
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
</pre>

and the following call

><pre>
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
</pre>

## Limitation of the Cacher implementation

* currently suppose that we call always with the same value.
* hold a hashmap rather a single value ?

## Capturing the environment with closures

><pre>
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
</pre>

Traits:

* _FnOnce_ consumes the variables it captures
* _FnMut_ can change the environment because it mutably borrows values
* _Fn_ borrows values from the environment immutably

* If you want to force the closure to take ownership of the values, it uses in the env, you can use the _move_ keyword before the parameter list

Not working:

><pre>
fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
</pre>

# Iterators

><pre>
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
for val in v1_iter {
    println!("Got: {}", val);
}
</pre>

## The _Iterator_ Trait and the _next_ Method

All iterators implement a trait named _Iterator_, defined in the standard library. Def of the trait looks like this:  
(type Item is an associated type with this trait)
><pre>
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}
</pre>

## Methods that consume the iterator

methods that call _next_ are called _consuming adaptors_, because calling them uses up the iterator. Example: sum() method

><pre>
 #[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
</pre>

## methods that produce other iterators

Other methods on _Iterator_ trait -> _iterator adaptors_, allow you to change iterators into different kinds of iterators.

Will generate an error:

><pre>
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1);
</pre>

><pre>
warning: unused `std::iter::Map` which must be used: iterator adaptors are lazy
and do nothing unless consumed
</pre>


Fixed:

><pre>
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
</pre>

## Using closures that capture their environment

Can be used with _filter_ iterator adaptor

><pre>
 #[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
</pre>

## Creating our own iterators with the _Iterator_ Trait

></pre>
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
//
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
</pre>

Using it:

><pre>
 #[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
</pre>

## Using Other _Iterator_ Trait Methods

><pre>
 #[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
</pre>
