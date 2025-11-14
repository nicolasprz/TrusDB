mod config;
mod prompts;
pub mod repl;

mod sql_compilator;
mod virtual_machine;

use std::fs::OpenOptions;
use std::io::Write;

/// Builds logger for this project: loads .env file and initiates logger with default value, along
/// with a special format and writing in a log file.
fn build_logger() {
    dotenv::dotenv().ok();
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .unwrap();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(move |buf, record| {
            let log_line = format!(
                "[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            );

            // Écrire dans le fichier
            writeln!(&log_file, "{}", log_line).ok();

            // Écrire dans la console
            writeln!(buf, "{}", log_line)
        })
        .init();
}

fn main() {
    build_logger();
    repl::run_repl();
}
