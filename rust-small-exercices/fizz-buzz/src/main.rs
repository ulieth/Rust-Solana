fn fizz_buzz() {
  for x in 0..301 {
    if x % 3 == 0 {println!("fizz")}
    if x % 5 == 0 {println!("buzz")}
    if x % 3 == 0 && x % 5 == 0 {println!("fizz buzz")}
  }

}

fn main() {
    fizz_buzz();
}


//At the end print the number of times "fizz buzz" occurred.
