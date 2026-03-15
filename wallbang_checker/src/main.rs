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

fn bullet_count(damage: &mut f64, max_hp: &mut f64) -> i32 {
    let mut count:i32 = 0;
    while (*damage * count as f64) < *max_hp {
        count+=1;
    }
    
    return count
}

fn main() {
    let mut material:Material;
    let mut input:String = String::new();
    let mut damage:f64 = 0.0;
    let mut ak47 = Bullet {
        damage: 34.0, // Урон в тело в CS2
        caliber: 762,
    };
    let mut max_hp: f64 = 100.0;
    loop{
        let mut input = String::new();
        println!("Выберите материал:");
        io::stdin().read_line(&mut input).unwrap();
        input.trim();
        match input.trim() {
            "1" => material = Material::Wood,
            "2" => material = Material::Drywall,
            "3" => material = Material::Concrete, 
            "4" => material = Material::Steel,
            _ => continue,
        }
        match material {
            Material::Wood => damage = ak47.damage * 0.8,
            Material::Drywall => damage = ak47.damage * 0.95,
            Material::Steel => damage = ak47.damage * 0.2,
            Material::Concrete => damage = ak47.damage * 0.75, 
        }
        println!("Damage: {}", damage);
        println!("Times you need to shoot with such wallbangs to kill someone with {max_hp}hp: {}", bullet_count(&mut damage, &mut max_hp))
    }
    
}