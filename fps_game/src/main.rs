use colored::*;
use rand::RngExt;

struct Entity {
    id: i32,
    hp: i32,
    max_damage: i32,
    min_damage: i32,
}

fn random(min: &i32, max: &i32) -> i32{
    let mut rng = rand::rng();
    let mut randint: i32 = rng.random_range(*min..=*max);
    return randint
}

fn game(){
    println!("Welcome to fps_game!");
    println!("The game based on your luck!");
    println!("So good luck bro!");
    let mut player1: Entity = Entity{id: 1, hp: 100, max_damage: 60, min_damage: 20,};
    let mut player2: Entity = Entity{id: 2, hp: 100, max_damage: 60, min_damage: 20,};
    let mut turn: bool = true;
    let mut damage: i32;
    loop{
        
        while player1.hp > 0 && player2.hp > 0 {
            if turn{
                damage = random(&player2.min_damage, &player2.max_damage);
                println!("2 player dealt: {damage} damage");
                player1.hp -= damage;
                turn = false;
            }else{
                damage = random(&player1.min_damage, &player1.max_damage);
                println!("1 player dealt: {damage} damage");
                player2.hp -= damage;
                turn = true;

            }

        }
        if player1.hp <= 0 && player2.hp <= 0{
            println!("Draw!");
            break;
        }else if player1.hp <= 0 {
            println!("You lost!");
            break;
        }else{
            println!("You won");
            break;
        }
    }
}

fn main() {
    game();
}
