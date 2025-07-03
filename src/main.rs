mod net_analyzer;
use net_analyzer::{NetworkAnalyze, input};
use net_analyzer::{
    clear, compare, create, delete, help, list, load_from_file, read, save_to_file, update,
};
use std::collections::HashMap;

// ─────────────────────────────────────────────
// Основной цикл командной оболочки
// ─────────────────────────────────────────────
/// Главная функция программы NetAnalyzer.
/// Инициализирует базу данных сетей, загружает данные из `network.json` (если он существует),
/// затем входит в бесконечный цикл, обрабатывающий команды пользователя.
/// 
/// Поддерживает команды: `add`, `read`, `upd`, `del`, `list`, `cls`, `cmp`, `help`, `exit`.
/// 
/// Сохраняет изменения в файл `network.json` после операций `add`, `upd`, `del`, `cls`.
/// 
/// Точка входа в программу NetAnalyzer.
fn main() {
    let path: &'static str = "network.json";
    let mut db: HashMap<String, NetworkAnalyze> = load_from_file(path);

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║       👋 Добро пожаловать в NetAnalyzer!      ║");
    println!("╚═══════════════════════════════════════════════╝");
    help(); // Display help message on startup

    loop {
        let command: String = input("🌐 Введите команду (или 'help' для списка команд): ");

        match command.as_str() {
            "add" => {
                create(&mut db);
                save_to_file(&db, path);
            }
            "read" => read(&db),
            "upd" => {
                update(&mut db);
                save_to_file(&db, path);
            }
            "del" => {
                delete(&mut db);
                save_to_file(&db, path);
            }
            "list" => list(&db),
            "cls" => {
                clear(&mut db);
                save_to_file(&db, path);
            }
            "cmp" => compare(&db),
            "help" => help(),
            "exit" => {
                println!("👋 До свидания! Данные сохранены.");
                break;
            }
            _ => println!("❓ Неизвестная команда! Введите 'help' для просмотра доступных команд."),
        }
    }
}
