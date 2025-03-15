pub trait Fizzy {
  fn fizzy(&self) -> String;
}

impl Fizzy for i32 {
  fn fizzy(&self) -> String {

      match (self % 3, self % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => format!("{}", self),
      }

  }

}



fn main() {
    let mut counter = 0;
    for x in 1..=301 {
      let result = x.fizzy();
      println!("{}", x.fizzy());
      if result == "FizzBuzz" {
        counter += 1;
      }
    }
    println!("FizzBuzz occurred {} times.", counter);
}
