use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

  let mut inputs = HashMap::new();

  for (i, &value) in nums.iter().enumerate() {
    let complement = target - value;

    if let Some(&index) = inputs.get(&complement) {
      return vec![index as i32, i as i32]
    }

    inputs.insert(value, i);

  };
  vec![]
}

fn main() {

  println!("{:?}", two_sum(vec![2, 3, 4, 5,], 9));
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn sum() {
    let nums: Vec<i32> = vec![2, 3, 4, 5,];
    let target: i32 = 9;
    assert_eq!(vec![2, 3], two_sum(nums, target));
  }
}
