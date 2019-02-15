fn main() {

  let days = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "thenth",
    "eleventh",
    "twelfth"
  ] ;

  let presents = [
    "a partidge",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a laying",
    "seven swans a swimming",
    "eight maids a milking",
    "nine ladies dancing",
    "ten lords a leaping",
    "eleven pipers piping",
    "twelve drummers drumming"
  ] ;

  for num in 0..12 {
    println!("On the {} day of Chrismas", days[num]);
    println!("my true love sent to me:");
    if num == 0 {
      println!("{} in a pear tree", presents[num]);
    } else {
      for num2 in (0..num+1).rev() {
        if num2 == 0 {
          println!("and {} in a pear tree", presents[num2]);
        } else {
          println!("{}", presents[num2]);
        }
      }
    }
    println!("")
  }

}
