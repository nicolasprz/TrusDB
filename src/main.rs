mod config;
mod prompts;
pub mod repl;
pub mod utils;

mod sql_compilator;
mod virtual_machine;

use std::fs::OpenOptions;
use std::io::Write;
use utils::file_handler;

const DATABASE_DEFAULT_PATH: &str = "trusdb";
const DATABASE_NAME: &str = "TrusDB";

/// Builds logger for this project: loads .env file and initiates logger with default value, along
/// with a special format and writing in a log file.
fn build_logger() {
    dotenv::dotenv().ok();
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("trusdb.log")
        .unwrap();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(move |buf, record| {
            let log_line = format!(
                "[{} {} {}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            );

            // Écrire dans le fichier
            writeln!(&log_file, "{}", log_line).ok();

            // Écrire dans la console
            writeln!(buf, "{}", log_line)
        })
        .init();
}

fn create_database_if_not_exists() -> std::io::Result<file_handler::Database> {
    file_handler::Database::create(DATABASE_DEFAULT_PATH, DATABASE_NAME)
}

fn main() -> std::io::Result<()> {
    build_logger();
    let database = create_database_if_not_exists()?;
    repl::run_repl(database);
    Ok(())
}
