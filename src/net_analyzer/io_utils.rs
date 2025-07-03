use std::io::{self, Write};
use std::net::Ipv4Addr;

// ─────────────────────────────────────────────
// Вспомогательная функция для ввода с консоли
// ─────────────────────────────────────────────
/// Считывает строку ввода от пользователя с консоли, отображая заданный `prompt`.
///
/// ### Аргументы
///
/// * `prompt` - Строковый срез, отображаемый пользователю перед вводом.
///
/// ### Возвращает
///
/// `String` - Строка, введенная пользователем, с удаленными пробелами в начале и конце.
pub fn input(prompt: &str) -> String {
    let mut buf: String = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

// ─────────────────────────────────────────────
// Расчёт параметров подсети по IP и маске
// ─────────────────────────────────────────────
/// Запрашивает у пользователя IP-адрес и маску подсети, затем вычисляет
/// различные параметры подсети, такие как сетевой адрес, длину префикса,
/// количество хостов, минимальный и максимальный адреса хостов, и шлюз.
///
/// ### Возвращает
///
/// Кортеж, содержащий:
/// * `Ipv4Addr` - Введенный IP-адрес.
/// * `Ipv4Addr` - Введенная маска подсети.
/// * `Ipv4Addr` - Сетевой адрес.
/// * `Ipv4Addr` - Минимальный адрес хоста.
/// * `Ipv4Addr` - Максимальный адрес хоста.
/// * `Ipv4Addr` - Адрес шлюза.
/// * `u32` - Длина префикса.
/// * `u32` - Количество хостов.
///
/// ### Паника
///
/// Паникует, если введенный IP-адрес или маска подсети не являются действительными форматами IPv4.
pub fn get_network() -> (
    Ipv4Addr,
    Ipv4Addr,
    Ipv4Addr,
    Ipv4Addr,
    Ipv4Addr,
    Ipv4Addr,
    u32,
    u32,
) {
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   ⚙️ Введите IP-адрес (можно с префиксом /)   ║");
    println!("╚═══════════════════════════════════════════════╝");
    let ip_input: String = input("🔹 Введите IP-адрес (например 192.168.0.1 или 192.168.0.1/24): ");
    println!("═════════════════════════════════════════════════\n");

    let (ip, mask, prefix_len): (Ipv4Addr, Ipv4Addr, u32) = if ip_input.contains('/') {
        // CIDR-формат: IP/префикс
        let parts: Vec<&str> = ip_input.split('/').collect();
        if parts.len() != 2 {
            panic!("❌ Неверный формат IP/префикс!");
        }
        let ip: Ipv4Addr = parts[0].parse().expect("❌ Неверный IP-адрес!");
        let prefix_len: u32 = parts[1].parse().expect("❌ Неверная длина префикса!");

        if prefix_len > 32 {
            panic!("❌ Префикс должен быть от 0 до 32!");
        }

        let mask_u32: u32 = if prefix_len == 0 {
            0
        } else {
            u32::MAX << (32 - prefix_len)
        };

        let mask: Ipv4Addr = Ipv4Addr::from(mask_u32);
        (ip, mask, prefix_len)
    } else {
        // Классический формат: IP и маска
        let ip: Ipv4Addr = ip_input.parse().expect("❌ Неверный IP-адрес!");
        let mask_str: String = input("🔹 Введите маску подсети (например 255.255.255.0): ");
        let mask: Ipv4Addr = mask_str.parse().expect("❌ Неверная маска!");
        let prefix_len: u32 = u32::from(mask).count_ones();
        (ip, mask, prefix_len)
    };

    // Далее как раньше:
    let ip_u32: u32 = u32::from(ip);
    let mask_u32: u32 = u32::from(mask);
    let network_u32: u32 = ip_u32 & mask_u32;
    let network: Ipv4Addr = Ipv4Addr::from(network_u32);
    let host_count: u32 = if prefix_len < 31 {
        2u32.pow(32 - prefix_len) - 2
    } else {
        0
    };
    let min_host: Ipv4Addr = Ipv4Addr::from(network_u32 + 1);
    let broadcast_u32: u32 = network_u32 | !mask_u32;
    let max_host: Ipv4Addr = if host_count > 0 {
        Ipv4Addr::from(broadcast_u32 - 1)
    } else {
        network
    };
    let gateway: Ipv4Addr = min_host;

    (
        ip, mask, network, min_host, max_host, gateway, prefix_len, host_count,
    )
}
