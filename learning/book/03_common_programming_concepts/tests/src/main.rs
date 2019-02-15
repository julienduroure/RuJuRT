fn main() {
    println!("Hello, world!");

    // constant
    const MAX_POINTS: u32 = 10_000;
    println!("Constant is {}", MAX_POINTS);

    // shadowing
    let x = 5;
    let x = x + 1;
    println!("Value is {}", x) ;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("2nd value is {}", tup.1);

    // fonction call
    let y = five() ;
    println!("Value is {}", y);

    let number = 6 ;
    if number % 4 == 0 {
      println!("Number is divisible by 4");
    } else if number % 3 == 0 {
      println!("Number is divisible by 3");
    } else if number % 2 == 0 {
      println!("Number is divisible by 2");
    } else {
      println!("Number is not divisible by 4, 3 or 2");
    }

    for number in (1..4).rev() {
      println!("{}", number);
    }
    println!("Liftoff!");

}

fn five() ->i32 {
  5
}
