pub mod repl;

mod prompts;
mod tokenizer;
mod parser;

fn main() {
    repl::run_repl();
}
