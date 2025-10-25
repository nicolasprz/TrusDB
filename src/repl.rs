use crate::prompts;
use std::io;
use std::io::Write;
use thiserror::Error;

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

#[derive(Error, Debug)]
enum TokenizingError {
    #[error(
        "Unexpected keyword found after '{word_before:?}': expected '{expected_word_after:?}', found '{found_word_after:?}'"
    )]
    KeywordNotFound {
        word_before: String,
        expected_word_after: String,
        found_word_after: String,
    },
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
    Operator,
    ColumnName(String),
    Expression(String),
}

struct Token {
    token_type: TokenType,
    content: String,
}

impl Token {
    fn new(token_type: TokenType, content: String) -> Token {
        Token {
            token_type,
            content,
        }
    }
}

fn tokenize_user_input(user_input: &str) {
    let tokens: Vec<Token> = Vec::new();
    let mut word_iter = user_input.split_whitespace().peekable();
    while let Some(word) = word_iter.next() {
        if let Some(&next) = word_iter.peek() {
            let some_token = build_token(word, next);
        }
    }
}

fn build_token(word: &str, next: &str) -> Result<Token, TokenizingError> {
    match word.to_lowercase().as_str() {
        "create" => generate_multiple_words_token(
            TokenType::Command(CommandType::CreateTable),
            word,
            next,
            "table",
        ),
        "select" => Ok(Token::new(
            TokenType::Command(CommandType::Select),
            word.to_string(),
        )),
        "insert" => generate_multiple_words_token(
            TokenType::Command(CommandType::InsertInto),
            word,
            next,
            "into",
        ),
        "update" => Ok(Token::new(TokenType::Command(CommandType::Update), word.to_string())),
        "delete" => Ok(Token::new(TokenType::Command(CommandType::Delete), word.to_string()))
    }
}

fn generate_multiple_words_token(
    output_token_type: TokenType,
    current_word: &str,
    word_after: &str,
    expected_word_after: &str,
) -> Result<Token, TokenizingError> {
    if word_after == expected_word_after {
        let content = format!("{}{}", current_word, word_after);
        Ok(Token::new(output_token_type, content))
    } else {
        Err(TokenizingError::KeywordNotFound {
            word_before: current_word.to_string(),
            expected_word_after: expected_word_after.to_string(),
            found_word_after: expected_word_after.to_string(),
        })
    }
}
