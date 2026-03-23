use std::{fs, io, path};

fn read_dir(path:&str){
    match fs::read_dir(&path){
        Ok(paths) => {
            for path in paths {
                println!("{:?}", path.unwrap().path());
            }
        },
        Err(err) => println!("{err}"),
    }
}
fn mk_dir(path:&str, name:&str){
    let mut path_name = format!("{path}/{name}");
    match fs::create_dir(&path_name) {
        Ok(_) => println!("Sucessfully created folder {name} at {path}"),
        Err(err) => println!("{err}")
    }    
}
fn read_file(path:&str){
    match fs::read_to_string(path){
        Ok(value) => {
            println!("Sucess read!");
            println!("{value}");
        },
        Err(err) => println!("{err}"),
    }
}
fn mk_file(path:&str, name:&str){
    let mut path_file:String = format!("{path}/{name}");
    match fs::File::create(&path_file) {
        Ok(_) => println!("Sucessfully created file {name} at {path}"),
        Err(err) => println!("{err}"),
    }
    
    
}
fn explorer(){
    loop{
        let mut input:String = String::new();
        let mut choose: i32 = 0;
        let mut path: String = String::new();
        let mut name: String = String::new();
        println!("Explorer cli utility");
        println!(r#"
        1. Read dir 
        2. Make dir
        3. Read file
        4. Make file"#);
        io::stdin().read_line(&mut input);
        choose = input.trim().parse().expect("Not a number!");
        
        match choose {
            1 => {
                println!("Enter path: ");
                input.clear();
                io::stdin().read_line(&mut input);
                path = input.trim().to_string();
                read_dir(&path);
            },
            2 => {
                println!("Enter path: ");
                input.clear();
                io::stdin().read_line(&mut input);
                path = input.trim().to_string();
                println!("Enter name: ");
                input.clear();
                io::stdin().read_line(&mut input);
                name = input.trim().to_string();
                mk_dir(&path,&name);
            },
            3 => {
                println!("Enter path: ");
                input.clear();
                io::stdin().read_line(&mut input);
                path = input.trim().to_string();
                read_file(&path);
            },
            4 => {
                println!("Enter path: ");
                input.clear();
                io::stdin().read_line(&mut input);
                path = input.trim().to_string();
                println!("Enter name: ");
                input.clear();
                io::stdin().read_line(&mut input);
                name = input.trim().to_string();
                mk_file(&path,&name);
            },
            _ => println!("Invalid number!"),
        }
        input.clear();
        path.clear();
        name.clear();
        choose = 0;

    }
}
fn main(){
    explorer();
}