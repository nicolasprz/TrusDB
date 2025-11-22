use crate::prompts;
use crate::sql_compilator::parser;
use crate::sql_compilator::tokenizer;
use crate::utils::file_handler;
use crate::virtual_machine::instruction_processor;
use std::io::Write;
use std::path::PathBuf;

const CONFIG_PATH: &str = "etc/config.toml";

pub fn run_repl(mut database: file_handler::Database) {
    let mut buffer: String = String::new();
    prompts::print_welcome_prompt();
    loop {
        print!("> ");
        std::io::stdout()
            .flush()
            .expect("Error flushing standard output");
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error read user input");
        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("exit") {
            break;
        }
        buffer.push_str(trimmed_input);
        buffer.push('\n');
        log::debug!("Content of buffer:\n{}", buffer);
        process_user_request(&buffer, &mut database);
    }
}

fn process_user_request(buffer: &str, database: &mut file_handler::Database) {
    let tokens: Vec<tokenizer::Token> =
        tokenizer::tokenize_user_input(buffer).expect("Error tokenizing input");
    log::debug!("{tokens:#?}");

    let parser: parser::Parser = parser::Parser::new(&tokens);
    let some_instruction: Option<parser::Instruction> =
        parser.parse_tokens().expect("Error while parsing tokens");
    log::debug!("{some_instruction:#?}");

    if let Some(instruction) = some_instruction {
        let mut query_processor = instruction_processor::InstructionProcessor::new(
            instruction,
            &PathBuf::from(CONFIG_PATH),
            database,
        )
        .expect("Could not create query processor");
        query_processor.process_instruction().expect("Error while processing instruction");
    } else {
        log::info!("Did not find any instruction to process");
    }
}
