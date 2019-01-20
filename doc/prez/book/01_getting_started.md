---
slug: 01-getting-started
title: Getting Started
page: True
menu: [book]
---

# Installation (linux)

>curl https://sh.rustup.rs -sSf | sh

# Updating

>rustup update

# Uninstall

>rustup self uninstall

# Various stuff

>* rustc --version
>* rustup doc

# Hello World program

* Code
> fn main() {  
> &nbsp;&nbsp;&nbsp;&nbsp;println!("Hello world!");  
>}  

println! is here a macro (because of !)

* Compile
> rustc main.rs  
>./main

# Cargo
Is the build system and package manager

> cargo new hello_cargo

# Building & running a cargo Project

Compile on target dir:
> cargo build

Compile & Run:
> cargo run

Check that code can compile (but doesn't do it):
> cargo check

Building for release:
> cargo build --release

File Cargo.lock:  
This file keeps track of the exact versions of dependencies in your project
