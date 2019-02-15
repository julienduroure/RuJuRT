use std::io;

fn main() {
    println!("Enter a temperature in Celcius");

    let mut celcius = String::new();

    io::stdin().read_line(&mut celcius)
      .expect("Failed to read line");

    let celcius: f64 = celcius.trim().parse()
      .expect("This is not a number");

    let fahr: f64 = (celcius * 9.0 / 5.0) + 32.0;
    println!("{}°C is {}° Fahrenheit", celcius, fahr);

}
