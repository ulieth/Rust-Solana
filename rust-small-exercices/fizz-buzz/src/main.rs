
fn main() {
  println!("{}","Welcome");
  let mut counter: u32 = 0;
  for ii in 1..=301 {
    match (ii % 3, ii % 5) {
      (0, 0) => {
        println!("FizzBuzz");
        counter += 1;

      }
      (0, _) => println!("Fizz"),
      (_, 0) => println!("Buzz"),
      _ => {}
    }
  }
  println!("{}",counter);


}
