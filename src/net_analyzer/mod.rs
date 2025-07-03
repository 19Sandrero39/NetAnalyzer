//! Главный модуль `net_analyzer`, объединяющий подсистемы: работу с сетью, базой данных и вводом пользователя.

mod db;
mod io_utils;
mod net;

pub use db::{
    clear, compare, create, delete, help, list, load_from_file, read, save_to_file, update,
};
pub use io_utils::{get_network, input};
pub use net::NetworkAnalyze;
