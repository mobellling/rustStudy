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
        println!("sss{a}");
        let a: u32 = match a.trim().parse() {
            Ok(b) => b,
            Err(_) => {
                println!("请输入合法数字");
                continue;
            }
        };
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
