---
slug: 07-packages-crates-modules
title: Packages, Crates & Modules
page: True
menu: [book]
---

# Summary

* A crate is a library or a binary
* The _create root_ is a source file that is used to know how to build a crate
* A package has a _Cargo.toml_ that describe how to build one or more crates. At most one crate in a package can be a library.

# Package

* cargo new --> This create a package

* If there is a _src_ directory containing a _main.rs_ in the same directory as a package's _Cargo.toml_ --> This is a binary crate with the same name as the package, and _src/main.rs_ is the _crate root_

* If there is a _src_ directory containing a _lib.rs_ in the same directory as a package's _Cargo.toml_ --> This is a library crate with the same name as the package, and _src/lib.rs_ is the _crate root_

* The crate root files are passed by Cargo to rustc to actually build the library or binary

* A package can contain zero or one library crates and as many binary crates as you’d like.

* There must be at least one crate (either a library or a binary) in a package.

* If a package contains both _src/main.rs_ and _src/lib.rs_, then it has two crates: a library and a binary, both with the same name. If we only had one of the two, the package would have either a single library or binary crate.

* A package can have multiple binary crates by placing files in the _src/bin_ directory: each file will be a separate binary crate

# Modules

We are going to talk about:

* Modules, a way to organize code and control the privacy of paths
* Paths, a way to name items
* _use_ keyword to bring a path into scope
* _pub_ to make items public
* Renaming items with _as_
* Using external packages
* Nested paths to clean up large _use_ lists
* Using the _glob_ operator to bring everything in a module into scope
* How to split modules into individual files

## modules

* you can define function inside modules
* you can nest modules inside of other modules
* refer to an item in a module tree, you use its path

## Paths

* A path can take two forms:
    * an _absolute path_ starts from a _crate root_ by using a crate name or a literal crate
    * a _relative path_ starts from the current module and uses _self_, _super_, or an identifier in the current module

* Both absolute and relative paths are followed by one or more identifiers separated by _::_


><pre>
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}
fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();
    // Relative path
    sound::instrument::clarinet();
}
</pre>

## Pub

* make an item public, usable from outside the current module
* making the module public does not make its contents public

* Use _super_ to refer to parent (equivalent of .. in a filesystem)

* pub struct --> Struct is public, but not fields.
* structs: Some fields can be publics, other privates

* When some fields of a struct are privates, the struct needs to provide a public associated function that constructs an instance of Vegetable (we’ve used the conventional name _new_ here)

* If an Enum is public --> all of its variants are public

><pre>
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
fn main() {
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
</pre>

* One more case where _pub_ is used --> See after, linked to _use_ keyword

## Use

* Adding use and a path in a scope is similar to creating a symbolic link in the filesystem

><pre>
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}
use self::sound::instrument;
fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
</pre>

Idiomatic:
* for functions:
    * specify the function’s parent module with _use_
    * specify the parent module when calling the function
* for structs, enums & other items:
    * specifying the full path to the item with _use_
* Except when two items have same name


## As

* when 2 items have same name

><pre>
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
}
fn function2() -> IoResult<()> {
}
</pre>

## Re-exporting names with _pub use_

* bringing an item into scope
* but also making that item available for others to bring into their scope


><pre>
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}
mod performance_group {
    pub use crate::sound::instrument;
    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}
fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
</pre>

## Using external packages

* Need to be listed in _Cargo.toml_
* Except _std_ that is shipped with rust
* std:  an absolute path starting with _std_, the name of the standard library crate

## Nested Paths for Cleaning Up Large _use_ Lists

* Instead of:

><pre>
use std::cmp::Ordering;
use std::io;
</pre>

* Use:

><pre>
use std::{cmp::Ordering, io};
</pre>

* Instead of:

><pre>
use std::io;
use std::io::Write;
</pre>

* Use:

><pre>
use std::io::{self, Write};
</pre>

## Glob operator

><pre>
use std::collections::*;
</pre>

## Separating Modules into Different Files

* use same name file / module
* launch content of a module using _use module_name;_
* use directory when using _use module_name_ in a module file
