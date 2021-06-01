struct Round {
  width: u32,
  height: u32,
}

trait Descriptive {
  fn area(&self) -> u32;
}

fn main() {
  let rect1 = Round {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );
}

impl Descriptive for Round {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
