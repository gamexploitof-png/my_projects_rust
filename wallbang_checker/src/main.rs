use std::io;

enum Material {
    Wood,
    Drywall, 
    Steel,  
    Concrete,
}

struct Bullet {
    damage: f64,
    caliber: i32,
}

fn main() {
    let mut material:Material;
    let mut input:String = String::new();
    let mut damage:f64 = 0.0;
    let mut ak47 = Bullet {
        damage: 34.0, // Урон в тело в CS2
        caliber: 762,
    };
    println!("Выберите материал:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim();
    match input.trim() {
        "1" => material = Material::Wood,
        "2" => material = Material::Drywall,
        "3" => material = Material::Concrete, 
        "4" => material = Material::Steel,
        _ => panic!("ТЫ ОБКУРЕННЫЙ?"),
    }
    match material {
        Material::Wood => damage = ak47.damage * 0.8,
        Material::Drywall => damage = ak47.damage * 0.95,
        Material::Steel => damage = ak47.damage * 0.2,
        Material::Concrete => damage = ak47.damage * 0.75, 
        _ => panic!("ТЫ ОБКУРЕННЫЙ?"),
    }
    println!("Damage: {}", damage)

}