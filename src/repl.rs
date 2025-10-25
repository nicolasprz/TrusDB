use crate::prompts;
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
    }
    println!("Content of buffer:\n{}", buffer);
}

enum CommandType {
    CreateTable,
    Select,
    InsertInto,
    Update,
    Delete,
}

enum Data {
    Float(f32),
    Integer(i32),
    Text(String),
    Bool(bool),
}

enum TokenType {
    Command(CommandType),
    ColumnName(String),
    Data,
}

struct Token {
    token_type: TokenType,
    content: String,
}
