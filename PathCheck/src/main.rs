use std::path::Path;
use colored::*; // Не забудь добавить colored = "2.1" в Cargo.toml

struct PathCheck {
    path: String,
    description: String,
}

// Функция теперь принимает Вектор структур по ссылке (&)
// Мы используем ссылку, чтобы не "сжигать" данные, если они понадобятся позже
fn check_n_print(paths: &Vec<PathCheck>) {
    println!("{:<20} | {:<15} | {}", "PATH".bold(), "STATUS".bold(), "DESCRIPTION".bold());
    println!("{}", "-".repeat(70));

    for item in paths {
        let exists = Path::new(&item.path).exists();
        
        // Форматируем статус с цветом
        let status = if exists {
            "  OK  ".green().on_black().bold()
        } else {
            "MISSING".red().on_black().bold()
        };

        println!("{:<20} | {:<15} | {}", item.path, status, item.description);
    }
}




fn main() {
    let mut paths: Vec<PathCheck> = Vec::new();
    
    paths.push(PathCheck { 
        path: "/proc/uptime".to_string(), 
        description: "Ядро Linux работает".to_string() 
    });
    paths.push(PathCheck { 
        path: "/etc/hostname".to_string(), 
        description: "Имя твоего ПК".to_string() 
    });
    paths.push(PathCheck { 
        path: "/var/log/syslog".to_string(), 
        description: "Системные логи".to_string() 
    });
    // Попробуй добавить путь, которого точно нет, чтобы увидеть красный цвет
    paths.push(PathCheck { 
        path: "/non/existent/path".to_string(), 
        description: "Эта проверка должна провалиться".to_string() 
    });

    check_n_print(&paths);
}