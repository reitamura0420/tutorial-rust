pub fn run() {
  let mut list = vec![1, 2, 5, 10, 9, 2, 1, 4, 6, 2, 8, 15, 8];

  println!("{}", calculate_mean(&list));
  println!("{}", calculate_median(&mut list));
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
  bubble_sort(list)[count]
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
