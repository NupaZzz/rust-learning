use std::io::{self, Write};

fn math_operation(input1: &str, input2: &str, operation: &str) -> Result<f64, String> {
    let num1 = input1.trim().parse::<f64>().map_err(|_| "Некорректное первое число")?;
    let num2 = input2.trim().parse::<f64>().map_err(|_| "Некорректное второе число")?;

    match operation.trim() {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Деление на ноль".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Неверная операция".to_string()),
    }
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operation = String::new();

    print!("Введите первое число: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).expect("Ошибка ввода");

    print!("Введите второе число: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).expect("Ошибка ввода");

    print!("Введите операцию (+, -, *, /): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut operation).expect("Ошибка ввода");

    match math_operation(&input1, &input2, &operation) {
        Ok(result) => println!("Результат: {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }
}
