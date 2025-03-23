// Given a vector of integers, compute the running sum where each element
// is the sum of all previous elements including itself.

// See if you can figure out a "Rusty" way to do it using Rust's iterators and methods

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
  println!("{:p}", &nums[..]);
 let cumsum: Vec<i32> = nums.into_iter().scan(0, |acc, x| {
    *acc += x;
    Some(*acc)
 })
 .collect();
 println!("{:p}", &cumsum[..]);
 cumsum
}

fn main() {
  println!("{:?}", running_sum(vec![1, 1, 1, 1, 1]));
}
