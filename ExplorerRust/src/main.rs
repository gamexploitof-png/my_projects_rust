use std::{fs, io, path::PathBuf};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "daun", about = "Explorer Cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>, // Опционально для работы без аргументов
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "viewfile")]
    ViewFile { path: String },
    #[command(name = "createfile")]
    CreateFile { path: String, name: String },
    #[command(name = "deletefile")]
    DeleteFile { path: String, name: String },
    #[command(name = "viewdir")]
    ViewDir { path: String },
    #[command(name = "createdir")]
    CreateDir { path: String, name: String },
    #[command(name = "deletedir")]
    DeleteDir { path: String, name: String },
}

// --- Улучшенные функции ---

fn read_dir(path: &str) {
    match fs::read_dir(path) {
        Ok(paths) => {
            for entry in paths.flatten() { // Безопасно распаковываем
                println!("{:?}", entry.path());
            }
        }
        Err(err) => eprintln!("Ошибка: {err}"),
    }
}

fn mk_dir(path: &str, name: &str) {
    let full_path = PathBuf::from(path).join(name);
    match fs::create_dir(&full_path) {
        Ok(_) => println!("Успешно создана папка: {:?}", full_path),
        Err(err) => eprintln!("Ошибка: {err}"),
    }
}

fn read_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(value) => {
            println!("Успешное чтение!\n---");
            println!("{value}");
        }
        Err(err) => eprintln!("Ошибка: {err}"),
    }
}

fn mk_file(path: &str, name: &str) {
    let full_path = PathBuf::from(path).join(name);
    match fs::File::create(&full_path) {
        Ok(_) => println!("Успешно создан файл: {:?}", full_path),
        Err(err) => eprintln!("Ошибка: {err}"),
    }
}

fn rm_dir(path: &str, name: &str) {
    let full_path = PathBuf::from(path).join(name);
    match fs::remove_dir(&full_path) {
        Ok(_) => println!("Успешно удалена папка: {:?}", full_path),
        Err(err) => eprintln!("Ошибка: {err}"),
    }
}

fn rm_file(path: &str, name: &str) {
    let full_path = PathBuf::from(path).join(name);
    match fs::remove_file(&full_path) {
        Ok(_) => println!("Успешно удален файл: {:?}", full_path),
        Err(err) => eprintln!("Ошибка: {err}"),
    }
}

// --- Основная логика ---

fn explorer() {
    let cli = Cli::parse();

    // 1. ПРОВЕРКА АРГУМЕНТОВ (CLI MODE)
    // Используем if let, так как команда опциональна
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::ViewFile { path } => read_file(&path),
            Commands::CreateFile { path, name } => mk_file(&path, &name),
            Commands::DeleteFile { path, name } => rm_file(&path, &name),
            Commands::ViewDir { path } => read_dir(&path),
            Commands::CreateDir { path, name } => mk_dir(&path, &name),
            Commands::DeleteDir { path, name } => rm_dir(&path, &name),
        }
        return; // Завершаем программу, если была команда
    }

    // 2. ИНТЕРАКТИВНЫЙ РЕЖИМ (LOOP MODE)
    let mut input = String::new();
    loop {
        println!("\nExplorer cli utility");
        println!("1. Read dir | 2. Make dir | 3. Read file | 4. Make file | 5. Delete dir | 6. Delete file | 0. Exit");
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let choose: i32 = input.trim().parse().unwrap_or(-1);

        if choose == 0 { break; }
        if choose < 1 || choose > 6 {
            println!("Invalid choice!");
            continue;
        }

        println!("Enter path: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let path = input.trim().to_string();

        match choose {
            1 => read_dir(&path),
            3 => read_file(&path),
            2 | 4 | 5 | 6 => {
                println!("Enter name: ");
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read");
                let name = input.trim().to_string();
                
                match choose {
                    2 => mk_dir(&path, &name),
                    4 => mk_file(&path, &name),
                    5 => rm_dir(&path, &name),
                    6 => rm_file(&path, &name),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    explorer();
}