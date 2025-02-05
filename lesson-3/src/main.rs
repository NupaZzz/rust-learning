use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!{"Я загадал число от 1 до 100. У тебя 6 попыток."}

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
        while attempts < 6 {
            print!("Попытка {}. Введите ваше предположение: ", attempts + 1);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let guess: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Пожалуйста, введите число.");
                    continue;
                }
            };
            attempts += 1;
            if guess < secret_number {
                println!("Больше!");
            } else if guess > secret_number {
                println!("Меньше!");
            } else {
                println!("Победа! Ты угадал число {} за {} попыток!", secret_number, attempts);
                break;
            }
        }
        if attempts >= 6 {
            println!("Ты проиграл! Загаданное число было: {}", secret_number);
        }

}
