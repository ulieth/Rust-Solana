// Given a vector of integers, compute the running sum where each element
// is the sum of all previous elements including itself.

// See if you can figure out a "Rusty" way to do it using Rust's iterators and methods

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
  let mut result = Vec::new();
  let mut iterator = nums.iter();
  let mut sum = 0;
  while let Some(&num) = iterator.next() {
      sum += num;
      result.push(sum);
  }
  result
}

fn main() {
  println!("{:?}", running_sum(vec![1, 1, 1, 1, 1]));
}
