extern crate rand;

use rand::{thread_rng, Rng};

use rand::distributions::{Alphanumeric, Standard, Uniform};
use rand::prelude::Distribution;
use std::iter::Iterator;

fn main() {
    generate_rand();
    range_rand();
    custom_rand();
    char_rand();
    password_rand();
}

fn generate_rand() {
    let mut rng = rand::thread_rng(); // ジェネレータ。

    let n1: u8 = rng.gen(); // 型の指定ができる。
    let n2: u16 = rng.gen(); // 整数は型の範囲内で生成される。
    let n3: f64 = rng.gen(); // 浮動小数点数は0から1の間（ただし1は含まない）の値が生成される。

    println!("Random u8: {}", n1);
    println!("Random u8: {}", n2);
    println!("Random u8: {}", n3);
    println!("Random u32 {}", rng.gen::<u32>());
}

fn range_rand() {
    let mut rng = rand::thread_rng();
    let die = Uniform::<i32>::from(1..7); // 同じ範囲の乱数を繰り返すならUniformの方が早い

    let n1: i32 = rng.gen_range(0..10); // 整数を指定したらi32
    let n2: f64 = rng.gen_range(0.0..10.0); // 小数を指定したらf64

    println!("integer: {}", n1);
    println!("float: {}", n2);

    loop {
        let throw = die.sample(&mut rng);
        println!("roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

#[derive(Debug)] // fmt::Debugを継承できる。端的に言うとprintln!を使うことができる。
struct Point {
    x: i32,
    y: i32,
}

// ユーザ定義型をランダムに生成する時はStandardを継承させる
impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn custom_rand() {
    let mut rng = rand::thread_rng();
    let rand_tuple: (i32, bool, f64) = rng.gen();
    let rand_point: Point = rng.gen(); // gen()はsampleが呼ばれる

    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

fn char_rand() {
    let rng: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from) // u8からcharに変換する必要がある。https://docs.rs/rand/0.8.5/rand/distributions/trait.Distribution.html#method.sample_iter
        .collect();

    println!("string: {}", rng);
}

fn password_rand() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~"; // constは型の指定が必要
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            // This is safe because `idx` is in range of `CHARSET`
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect();

    println!("password: {:?}", password);
}
