use std::io;

fn main() {
    println!("Press Enter:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
