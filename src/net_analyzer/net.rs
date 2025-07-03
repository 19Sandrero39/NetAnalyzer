use std::net::Ipv4Addr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkAnalyze {
    /// IP-адрес, введенный пользователем.
    pub ip: Ipv4Addr,
    /// Маска подсети, введенная пользователем.
    pub mask: Ipv4Addr,
    /// Сетевой адрес подсети (результат побитового И IP и маски).
    pub network: Ipv4Addr,
    /// Минимальный адрес хоста в подсети (первый доступный IP).
    pub min_host: Ipv4Addr,
    /// Максимальный адрес хоста в подсети (последний доступный IP перед широковещательным адресом).
    pub max_host: Ipv4Addr,
    /// Адрес шлюза по умолчанию (обычно, но не всегда, совпадает с `min_host`).
    pub gateway: Ipv4Addr,
    /// Длина префикса подсети (количество единичных битов в маске подсети).
    pub prefix_len: u32,
    /// Количество доступных хостов в подсети.
    pub host_count: u32,
}

impl NetworkAnalyze {
    /// Создает новый экземпляр `NetworkAnalyze` с заданными параметрами.
    ///
    /// ### Аргументы
    ///
    /// * `ip` - IP-адрес.
    /// * `mask` - Маска подсети.
    /// * `network` - Сетевой адрес.
    /// * `min_host` - Минимальный адрес хоста.
    /// * `max_host` - Максимальный адрес хоста.
    /// * `gateway` - Адрес шлюза по умолчанию.
    /// * `prefix_len` - Длина префикса.
    /// * `host_count` - Количество хостов.
    ///
    /// ### Возвращает
    ///
    /// `Self` - Новый экземпляр `NetworkAnalyze`.
    pub fn new(
        ip: Ipv4Addr,
        mask: Ipv4Addr,
        network: Ipv4Addr,
        min_host: Ipv4Addr,
        max_host: Ipv4Addr,
        gateway: Ipv4Addr,
        prefix_len: u32,
        host_count: u32,
    ) -> Self {
        Self {
            ip,
            mask,
            network,
            min_host,
            max_host,
            gateway,
            prefix_len,
            host_count,
        }
    }
}
