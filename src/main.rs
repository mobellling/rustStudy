use std::io;
fn function() -> i32 {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    print!("function called");
    return n;
}

fn main() {
    let a: i32 = function();
    println!("Hello, world!");
    println!("The value of a is: {}", a);
}