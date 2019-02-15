use std::io;

fn main() {
    println!("Enter a number");

    let mut val = String::new();

    io::stdin().read_line(&mut val)
      .expect("Failed to read line");

    let val = val.trim().parse()
      .expect("This is not an integer");

    println!("Fibonacci of {} is {}", val, fibo(val));
}

fn fibo(x: u32) -> u32 {
  if x <= 1 {
    x
  } else {
      fibo(x - 1) + fibo(x - 2)
  }

}
