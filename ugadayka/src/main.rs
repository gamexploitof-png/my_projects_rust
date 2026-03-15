use rand::Rng;      // трейт для работы с RNG
use rand::RngExt;   // трейт для random_range
use std::io;

fn greet() {
    println!("Welcome to the ugadayka game");
}

fn rand(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();        // создаём генератор
    rng.random_range(min..=max)       // генерируем число
}

fn print_history(history: &[i32]) {
    println!("Your guesses:");
    for guess in history {
        println!("{}", guess);
    }
}

fn game(secret: i32, history: &mut Vec<i32>) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let guess: i32 = input.trim().parse().unwrap();
        history.push(guess);

        if guess == secret {
            println!("You won!");
            break;
        } else if guess < secret {
            println!("Больше!");
        } else {
            println!("Меньше!");
        }
    }
}

fn main() {
    greet();
    let mut history: Vec<i32> = Vec::new();
    let secret = rand(1, 100);

    game(secret, &mut history);
    print_history(&history);
}