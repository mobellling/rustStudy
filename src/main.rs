use rand::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut a = String::new();
    let se: u32 = rand::rng().random_range(1..101);
    loop {
        a.clear();
        println!("请输入一个数字: ");
        io::stdin().read_line(&mut a).expect("err");
        let a: u32 = a.trim().parse().expect("err");
        match a.cmp(&se) {
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
        }
    }
}
