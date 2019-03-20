use::std::collections::HashMap;

// Length
fn get_len(vec: &Vec<i32>) -> i32 {
  vec.len() as i32
}

// Mean
fn get_mean(vec: &Vec<i32>) -> f64 {
  let mut m: f64 = 0.0;
  for i in vec {
    m = m + *i as f64;
  }

  m / vec.len() as f64
}

// Median
fn get_median(vec: &Vec<i32>) -> i32 {

    let mut vec_sorted = vec.to_vec();
    vec_sorted.sort();
    vec_sorted[((vec.len() - 1) as i32 / 2) as usize]
}

// Mode
fn get_mode(vec: &Vec<i32>) -> (i32, i32) {

  let mut map = HashMap::new();

  for i in vec {
    let count = map.entry(*i).or_insert(0);
    *count += 1;
  }

  let mut max = 0;
  let mut max_key = 0; // how initialisation can be done?
  for (key, value) in &map {
    if *value > max {
      max = *value ;
      max_key = *key;
    }
  }

  (max_key, max)

}

fn main() {
  // For this exercice, only odd len of vector !
  let vec = vec![1, 1, 4, 8, 10, 3, 4, 5, 6, 7, 0];

  let len = get_len(&vec);
  println!("length is {}", len);

  let mean = get_mean(&vec);
  println!("mean is {}", mean);

  let median = get_median(&vec);
  println!("median is {}", median);

  let max: (i32, i32);
  max = get_mode(&vec);
  println!("Most used value is {}, (used {} times)", max.0, max.1);
}
