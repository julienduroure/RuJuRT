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
