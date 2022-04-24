use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut list = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      list.push(line);
    };
  }
  list
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut list = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      list.push(line);
    };
  }
  list
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let mut f = File::open(config.filename)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  let result = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_insensitive(&config.query, &contents)
  };

  for line in result {
    println!("{}", line);
  }

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_insensitive(query, contents)
    );
  }
}
