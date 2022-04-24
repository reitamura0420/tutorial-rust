use std::collections::HashMap;

pub fn run() {
  let mut list = vec![1, 2, 5, 10, 9, 2, 1, 4, 6, 2, 8, 15, 8];

  println!("{}", calculate_mean(&list));
  println!("{}", calculate_median(&mut list));
  println!("{}", calculate_mode(&mut list));
}

fn calculate_mean(list: &Vec<i32>) -> i32 {
  let mut sum = 0;
  let mut count = 0;
  for num in list {
    sum = sum + num;
    count = count + 1;
  }
  sum / count
}

fn calculate_median(list: &mut Vec<i32>) -> i32 {
  let count = list.len();
  bubble_sort(list)[count - 1]
}

fn bubble_sort(list: &mut Vec<i32>) -> &Vec<i32> {
  for i in 0..list.len() {
    for j in 1..(list.len() - i) {
      if list[j] < list[j - 1] {
        list.swap(j, j - 1);
      }
    }
  }

  list
}

fn calculate_mode(list: &Vec<i32>) -> &i32 {
  let mut map = HashMap::new();
  let mut max_count = 0;
  let mut mode: &i32 = &0;
  for word in list {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  for (key, value) in map {
    if value > max_count {
      max_count = value;
      mode = key;
    }
  }
  mode
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_calculate_mean() {
    let list = vec![1, 2, 5, 10, 9, 2, 1, 4, 6, 2, 8, 15, 8];
    assert_eq!(calculate_mean(&list), 5);
  }

  #[test]
  #[ignore]
  fn test_calculate_median() {
    let mut list = vec![1, 2, 5, 10, 9, 2, 1, 4, 6, 2, 8, 15, 8];
    assert_eq!(calculate_median(&mut list), 15);
  }
}
