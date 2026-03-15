struct Masterpiece{
    title : String,
    composer : String,
    difficulty: i32,
}

fn main() {
    let mut masterpiece = Masterpiece{
        title : "Rush E".to_string(),
        composer : "Andrew Wrangell".to_string(),
        difficulty : 10,
    };
    println!("Произведение {} написал {}, сложность {}", masterpiece.title, masterpiece.composer, masterpiece.difficulty);
    if masterpiece.difficulty>=8{
        println!("Невозможно (Это Rush E!)");
    }else if masterpiece.difficulty>=4{
        println!("Средне (Нужно порепетировать)");
    }else{
        println!("Легко (Для начинающих)");
    }
    

}
