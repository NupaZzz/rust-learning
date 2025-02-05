use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Введите ваше имя: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Привет, {}!", input.trim());
}