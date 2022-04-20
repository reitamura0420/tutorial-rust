use std::collections::HashMap;

pub fn run() {
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  println!("{:?}", scores);

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);

  println!("{:?}", map);

  let color = String::from("Favorite color");

  println!("{:?}", map.get(&color));

  for (key, value) in &scores {
    println!("{} : {}", key, value);
  }

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);
  println!("{:?}", scores);

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(70);

  println!("{:?}", scores);

  let mut map = HashMap::new();
  let text = "hello happy world is world no1";

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}
