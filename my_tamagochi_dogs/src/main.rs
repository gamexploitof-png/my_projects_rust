use std::io::{self, Write};
mod dogs;

fn print_dogs(dogs: &Vec<dogs::Dog>) {
    println!("\n--- Dogs List ---");
    for (i, dog) in dogs.iter().enumerate() {
        println!("{}. Name: {}, Age: {}, Hunger: {}, Thirst: {}", 
                 i, dog.name, dog.age, dog.hunger, dog.thirst);
    }
}

fn main() {
    let mut dogs: Vec<dogs::Dog> = Vec::new();
    let mut input = String::new();

    loop {
        print_dogs(&dogs);

        println!(r#"
        ..MY TAMAGOTCHI DOGS..
         1. Добавить собаку
         2. Кормить собаку (Chicken)
         3. Поить собаку
         4. Удалить собаку
         5. Выход
         "#);
        
        print!("Choose: ");
        io::stdout().flush().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let choose: i32 = input.trim().parse().unwrap_or(0);

        match choose {
            1 => {
                input.clear();
                println!("Enter name: ");
                io::stdin().read_line(&mut input).unwrap();
                let name = input.trim().to_string();

                input.clear();
                println!("Enter age: ");
                io::stdin().read_line(&mut input).unwrap();
                let age: i32 = input.trim().parse().unwrap_or(0);

                dogs.push(dogs::Dog {
                    name,
                    age,
                    hunger: 50,
                    thirst: 50,
                });
            },
            2 => {
                if dogs.is_empty() { continue; }
                println!("Enter index:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let index: usize = input.trim().parse().unwrap_or(999);

                if let Some(dog) = dogs.get_mut(index) {
                    dog.feed(dogs::Food::Chicken);
                } else {
                    println!("Dog not found!");
                }
            },
            3 => {
                if dogs.is_empty() { continue; }
                println!("Enter index:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let index: usize = input.trim().parse().unwrap_or(999);

                if let Some(dog) = dogs.get_mut(index) {
                    dog.water(20);
                } else {
                    println!("Dog not found!");
                }
            },
            4 => {
                if dogs.is_empty() { continue; }
                println!("Enter index to delete:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let index: usize = input.trim().parse().unwrap_or(999);

                if index < dogs.len() {
                    dogs.remove(index);
                    println!("Removed.");
                } else {
                    println!("Invalid index!");
                }
            },
            5 => break,
            _ => println!("Invalid option!"),
        }
    }
}