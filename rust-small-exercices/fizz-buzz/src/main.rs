fn fizz_buzz() {
  let mut counter = 0;
  for x in 1..=301 {
    match (x % 3, x % 5) {
      (0, 0) => {
        counter += 1;
        println!("FizzBuzz");
      },
      (0, _) => println!("Fizz"),
      (_, 0) => println!("Buzz"),
      _ => println!("{}", x),
    }
  }
  println!("FizzBuzz occurred {} times.", counter);
}

fn main() {
    fizz_buzz();
}
