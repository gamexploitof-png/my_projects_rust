use std::{io, usize};

#[derive(Debug)]
struct Task {
    description: String,
    done: bool,
}


fn add(todo_list: &mut Vec<Task>, description: String) {
    let task = Task {
        description,
        done: false,
    };
    
    todo_list.push(task);
}


fn remove(todo_list: &mut Vec<Task>, index: usize) {
    if index < todo_list.len() {
        todo_list.remove(index);
    } else {
        println!("Нет задачи с таким номером!");
    }
}

fn mark_done(todo_list: &mut Vec<Task>, index: usize) {
    if index < todo_list.len() {
        
        todo_list[index].done = true;
    } else {
        println!("Нет задачи с таким номером!");
    }
}

fn print_list(todo_list: &Vec<Task>) {
    if todo_list.is_empty() {
        println!("Список пуст.");
    } else {
        for (i, task) in todo_list.iter().enumerate() {
            println!(
                "{}: {} [{}]",
                i + 1,
                task.description,
                if task.done { "✓" } else { "✗" }
            );
        }
    }
}


fn read_option() -> i32 {
    println!("\n1. Добавить туду");
    println!("2. Удалить туду");
    println!("3. Выполнить туду");
    println!("4. Выйти");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Ошибка ввода, попробуйте снова.");
            0
        }
    }
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();

    loop {
        println!("\nТекущий список задач:");
        print_list(&todo_list);

        let choice = read_option();

        match choice {
            1 => {
                
                let mut desc = String::new();
                println!("Введите описание задачи:");
                io::stdin().read_line(&mut desc).unwrap();
                add(&mut todo_list, desc.trim().to_string());
                desc.clear();
            }
            2 => {
                
                let mut idx = String::new();
                println!("Введите номер задачи для удаления:");
                io::stdin().read_line(&mut idx).unwrap();
                match idx.trim().parse::<usize>() {
                    Ok(i) => remove(&mut todo_list, i - 1), 
                    Err(_) => println!("Неверный ввод!"),
                }
                idx.clear();
            }
            3 => {
                let mut idx: String = String::new();
                println!("Введите номер задачи для выполнения: ");
                io::stdin().read_line(&mut idx).unwrap();
                match idx.trim().parse::<usize>() {
                    Ok(i) => mark_done(&mut todo_list, i - 1), 
                    Err(_) => println!("Неверный ввод!"),
                }
                idx.clear();
            }
            4  => {
                println!("Выход из программы.");
                break;
            }
            _ => println!("Неверный выбор!"),
        }
    }
}