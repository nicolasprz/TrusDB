pub mod repl;

mod prompts;
mod tokenizer;

fn main() {
    repl::run_repl();
}
