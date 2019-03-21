---
slug: 12-minigrep
title: "Command line program: minigrep"
page: True
menu: [book]
---
# Arguments

* get arguments

><pre>
use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
}

result:

><pre>
$ cargo run
--snip--
["target/debug/minigrep"]
$ cargo run needle haystack
--snip--
["target/debug/minigrep", "needle", "haystack"]
</pre>

* saving argument values in variables

><pre>
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
}
</pre>

# Reading a file

><pre>
use std::env;
use std::fs;
fn main() {
    // --snip--
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
</pre>

# Reading env variables

><pre>
use std::env;
let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
</pre>

# Writing to standard Error

><pre>
eprintln!("Application error: {}", e);
</pre>
