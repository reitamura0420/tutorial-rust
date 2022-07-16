extern crate clap;

use clap::{App, Arg};

fn main() {
    builder_style();
}

fn builder_style() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Rei Tamura")
        .about("Teaches argument parsing")
        .arg(
            Arg::with_name("file") // value_ofで識別する値
                .short('f') // 短いオプションの指定 charで指定する
                .long("file") // 長いオプションの指定
                .takes_value(true) // 値を取ることを指定する
                .help("A cool file"),
        )
        .arg(
            Arg::with_name("num")
                .short('n')
                .long("number")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt"); // unwrap_orにするとNoneの時に設定した値が入る
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        },
    }
}
