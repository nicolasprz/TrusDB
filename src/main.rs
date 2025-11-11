pub mod repl;
mod prompts;

mod sql_compilator;
mod virtual_machine;

fn main() {
    repl::run_repl();
}
