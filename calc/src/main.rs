use std::io;

fn greeting() {
    println!(r#"
 .o88b. db      d888888b       .o88b.  .d8b.  db       .o88b. db    db db       .d8b.  d888888b  .d88b.  d8888b. 
d8P  Y8 88        `88'        d8P  Y8 d8' `8b 88      d8P  Y8 88    88 88      d8' `8b `~~88~~' .8P  Y8. 88  `8D 
8P      88         88         8P      88ooo88 88      8P      88    88 88      88ooo88    88    88    88 88oobY' 
8b      88         88         8b      88~~~88 88      8b      88    88 88      88~~~88    88    88    88 88`8b   
Y8b  d8 88booo.   .88.        Y8b  d8 88   88 88booo. Y8b  d8 88b  d88 88booo. 88   88    88    `8b  d8' 88 `88. 
 `Y88P' Y88888P Y888888P       `Y88P' YP   YP Y88888P  `Y88P' ~Y8888P' Y88888P YP   YP    YP     `Y88P'  88   YD 
"#);
    println!("Добро пожаловать в калькулятор!\n");
}

fn add(first: f64, second: f64) -> f64 { first + second }
fn substract(first: f64, second: f64) -> f64 { first - second }
fn times(first: f64, second: f64) -> f64 { first * second }
fn divide(first: f64, second: f64) -> f64 { first / second }

fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        input.clear();
        io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Ошибка! Введите корректное число."),
        }
    }
}

fn read_operator() -> char {
    let mut input = String::new();
    loop {
        println!("Введите действие (+, -, *, /) или q для выхода: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
        let op = input.trim().chars().next();
        match op {
            Some('+' | '-' | '*' | '/' | 'q') => return op.unwrap(),
            _ => println!("Ошибка! Неверная операция."),
        }
    }
}

fn solve() {
    loop {
        let first_num = read_number("Введите число 1: ");
        let second_num = read_number("Введите число 2: ");

        let op = read_operator();
        if op == 'q' {
            println!("Выход из калькулятора.");
            break;
        }

        let result = match op {
            '+' => add(first_num, second_num),
            '-' => substract(first_num, second_num),
            '*' => times(first_num, second_num),
            '/' => {
                if second_num == 0.0 {
                    println!("Ошибка! Деление на ноль.");
                    continue;
                }
                divide(first_num, second_num)
            }
            _ => unreachable!(),
        };

        println!("Результат: {} {} {} = {}", first_num, op, second_num, result);
    }
}

fn main() {
    greeting();
    solve();
}