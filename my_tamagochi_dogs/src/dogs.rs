#[derive(Debug)]
pub enum Food {
    Chicken,
    Beef,
    Pork,
}

pub struct Dog {
    pub name: String,
    pub age: i32,
    pub hunger: i32,
    pub thirst: i32,
}

impl Dog {
    pub fn bark(&mut self) {
        println!("{} says woof!", self.name);
        self.hunger -= 10;
    }

    pub fn feed(&mut self, food: Food) {
        match food {
            Food::Chicken => {
                self.hunger += 15;
                println!("Fed {} chicken. Hunger: {}.", self.name, self.hunger);
            }
            Food::Beef => {
                self.hunger += 30;
                println!("Fed {} beef. Hunger: {}.", self.name, self.hunger);
            }
            Food::Pork => {
                self.hunger += 20;
                println!("Fed {} pork. Hunger: {}.", self.name, self.hunger);
            }
        }
    }

    pub fn water(&mut self, water_amount: i32) {
        self.thirst += water_amount;
        println!("{} drank water. Thirst: {}.", self.name, self.thirst);
    }
}