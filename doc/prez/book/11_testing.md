---
slug: 11-testing-md
title: Testing
page: True
menu: [book]
---


# Writing tests

* a test is a function that's annotated with the _test_ attribute
* Add _#[test]_ on the line before _fn_
* Run your tests with the _cargo test_ command
* A test failed if panic!


><pre>
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
</pre>

* Checking Results with _assert!_ (will panic if condition is False)

><pre>
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
</pre>

><pre>
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
    }
}
</pre>

* Some macro:
    * assert_eq!
    * assert_ne!

* For struct & enums --> need to implement _PartialEq_ and _Debug_
* Tip --> this is usually as straightforward as adding the _#[derive(PartialEq, Debug)]_ annotation to your struct or enum definition

* Adding custom failure messages

><pre>
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
</pre>

* Checking for panics with _should_panic_

><pre>
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
</pre>

* Using _Result< T, E >_ in tests

><pre>
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
</pre>

# Running tests

* cargo test
* If want to pass arguments to your program, separate with _--_:
    * cargo test --help displayes options of test
    * cargo test -- --help displays the options of your programs

* Running tests in parallel or Consecutively

By default, they run in parallel using threads
If you don't want: cargo test -- --test-threads=1

* Showing function output

cargo test -- --nocapture

* Running a subset of tests by name

cargo test test_name

* Filtering to run multiple tests

you can specify part of a test name, any test whose name matches that value will be run

* Ignoring some tests unless specifically requested

><pre>
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
</pre>

run only ignore tests: cargo test -- --ignored

# Test organization

2 kinds of tests:
    * unit tests
    * integration tests

## Unit tests

* Convention is to create a modules names tests in each file to contain the test functions and to annotate the module with cfg(test)

* Testing private function: This is possible

## Integration tests

* Need a tests directory, next to src
* Cargo will compile each of the files in tests dir as an individual crate

><pre>
use adder;
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
</pre>

* submodule in integration tests

To create a submodule not seen as integration test file:
    * create tests/common/mod.rs
    * call this module from other integration test file

><pre>
use adder;
mod common;
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
</pre>

* Integration tests for binary crates

You can't create integration tests.
Tip:

This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests can test the library crate with use to make the important functionality available. If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.
