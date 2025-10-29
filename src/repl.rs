use crate::parser;
use crate::prompts;
use crate::tokenizer;
use std::io;
use std::io::Write;

pub fn run_repl() {
    let mut buffer: String = String::new();
    prompts::print_welcome_prompt();
    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("Error flushing standard output");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error read user input");
        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("exit") {
            break;
        }
        buffer.push_str(trimmed_input);
        buffer.push('\n');
        println!("Content of buffer:\n{}", buffer);
        let tokens: Vec<tokenizer::Token> =
            tokenizer::tokenize_user_input(&buffer).expect("Error tokenizing input");
        println!("{tokens:#?}");
        let instruction: Option<parser::Instruction> =
            parser::parse_tokens(tokens).expect("Error while parsing tokens");
        println!("{instruction:?}");
    }
}
