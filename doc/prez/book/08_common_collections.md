---
slug: 08-common-collections
title: Common collections
page: True
menu: [book]
---

# Vectors

* Create a new empty vector:

></pre>
let v: Vec<i32> = Vec::new();
</pre>

* Using macro:

><pre>
let v = vec![1, 2, 3];
</pre>

* Update of vector

><pre>
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
</pre>

* Dropping

When a vector is dropped, all of its content is dropped, can be problematic when reference to elements of vector

><pre>
{
  let v = vec![1, 2, 3, 4];
} // out of scope


* Reading elements : reference or .get

><pre>
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];
println!("The third element is {}", third);
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
</pre>

* Reference & ownership

This will not compile:

><pre>
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0]; // immutable reference --> borrow ownership
v.push(6);
println!("The first element is: {}", first) // try to use here
</pre>

* Iterating

><pre>
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
</pre>

* Iterating mutable  
Need to dereference

><pre>
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
</pre>

* Use _Enum_ to store multiple type

><pre>
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
</pre>

* See [https://doc.rust-lang.org/std/vec/index.html](https://doc.rust-lang.org/std/vec/index.html)

# String

* Provided by _std_ library, is a growable, mutable, owned, UTF-8 encoded string type

* Creating a new string

><pre>
// empty
let mut s = String::new();
// initialisation
let data = "initial contents";
let s = data.to_string();
// the method also works on a literal directly:
let s = "initial contents".to_string();
// or
let s = String::from("initial contents");
</pre>

* Updating a string

    * Appending

><pre>
let mut s = String::from("foo");
s.push_str("bar");
// push --> single character
let mut s = String::from("lo");
s.push('l');
</pre>

    * Concatenation with _+_ or _format_

><pre>
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
// or
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
</pre>

    * Indexing into String

Rust strings don’t support indexing, because .len() returns nb of bytes, not nb of chars

    * Iterating over strings

><pre>
for c in "नमस्ते".chars() {
  println!("{}", c);
}
</pre>

# HashMap

* Creating a new HashMap

><pre>
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
</pre>

* Creation from vec of tuples

><pre>
use std::collections::HashMap;
let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
</pre>

* Ownership

    * For type that implement the _Copy_ trait, like _i32_, values are copied into the hashmap
    * For owned values like _String_, values will be moved and hashmap is the new owner of those values

><pre>
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
</pre>

    * If we insert references to values into a hashmap, the values won't be moved. The values that the references point to must be valid for at least as long as the hash map is valid. We’ll talk more about these issues in the “Validating References with Lifetimes” section in Chapter 10.

* Accessing values: _get_

    * _get_ returns an _Option<&v>_

><pre>
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");
let score = scores.get(&team_name);
</pre>

* Iterate

><pre>
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
</pre>

* Updating

3 ways of working

    * Overwriting a value

><pre>
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
println!("{:?}", scores);
</pre>

    * Only inserting a value if the key has no value (no change for existing ones)

><pre>
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
println!("{:?}", scores);
</pre>


    * Updating a value  based on old one

><pre>
use std::collections::HashMap;
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map);
</pre>
