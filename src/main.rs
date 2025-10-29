pub mod repl;

mod prompts;
mod tokenizer;
mod parser;
mod lookahead;

fn main() {
    repl::run_repl();
}
