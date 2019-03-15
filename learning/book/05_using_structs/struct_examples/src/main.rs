// By struct
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {

// by hand
  let width1 = 30;
  let height1 = 50;

  println!("The area is {}", area_hand(width1, height1));

  // by tuple
  let rect1 = (30, 50);
  println!("The area is {}", area_tuple(rect1));

  // By struct
  let rect = Rectangle { width: 30, height: 50};
  println!("The area is {}", area_struct(&rect));

}

// By hand
fn area_hand(width: u32, height: u32) -> u32 {
  width * height
}

// By tuple
fn area_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

// By struct
fn area_struct(rectangle:  &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
