---
slug: 09-error-handling
title: Error handling
page: True
menu: [book]
---


# Errors

* 2 types:
    * recoverable: have a type _Result<T, E>_
    * unrecoverable, have _panic!_ macro to stop the program

* No exceptions in Rust

# unrecoverable errors with _panic!_

* Default: _unwinding_ --> clean up data before ending program
* Other solution --> _abort_ --> end the program without cleaning up

* To abort on panic in releae mode, add in Cargo.toml file:

><pre>
[profile.release]
panic = 'abort'
</pre>

* Calling panic!

><pre>
fn main() {
  panic!("Crash & burn");
}
</pre>

* Using Backtrace

><pre>
fn main() {
  let v = vec![1, 2, 3];
  v[99];
}
</pre>

Call it with :

><pre>
RUST_BACKTRACE=1 cargo run
</pre>

Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here


# Recoverable errors with _Result_

* Result is defined as:  
(T & E are generic type parameters, see §10)

><pre>
enum Result<T, E> {
    Ok(T),
    Err(E),
}
</pre>

* Example

><pre>
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
</pre>

* Matching different errors

><pre>
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}
</pre>

In chapter §13, we will talk about _closure_

><pre>
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}
</pre>

* Shortcuts for panic on error: _unwrap_ and _expect_

    * unwrap --> if _Ok_, return the value inside _ok_. If _Err_, will call _panic!_
    * expect --> same, but let choose an error message

><pre>
use std::fs::File;
fn main() {
    let f   = File::open("hello.txt").unwrap();
    let f_2 = File::open("hello.txt").expect("Failed to open hello.txt");
}
</pre>

* Propagating errors

><pre>
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
</pre>

* Shortcut for propagating errors : _?_ operatot

><pre>
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
</pre>

Shorter:

><pre>
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
</pre>

Another way:

><pre>
use std::io;
use std::fs;
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
</pre>
