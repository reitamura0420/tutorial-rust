extern crate rand;

use rand::Rng;

use rand::distributions::Uniform;
use rand::prelude::Distribution;

fn main() {
    generate_rand();
    range_rand();
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
