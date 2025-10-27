use crate::tokenizer::{CommandType, Token, TokenType};
use std::collections::{HashMap, HashSet};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParsingError {
    #[error("First token for statement was not a valid command: '{found_content}'")]
    FirstTokenNotCommand { found_content: String },
    #[error("Expected token was not found")]
    TokenNotFound { expected: TokenType },
    #[error(
        "A character is missing in a token: '{missing_char}' is expected at position \
        {char_expected_position}, found '{found_content}' instead"
    )]
    MissingCharInToken {
        missing_char: char,
        char_expected_position: usize,
        found_content: String,
    },
}

type InstructionResult = Result<Vec<Instruction>, ParsingError>;

struct Instruction {
    base_command: CommandType,
    target_table: String,
    columns: Vec<Column>,
}

enum DataType {
    Float,
    Integer,
    Text,
    Bool,
}

struct Column {
    name: String,
    data_type: DataType,
    values: Vec<DataType>,
}

fn parse_tokens(tokens: Vec<Token>) -> InstructionResult {
    let mut instructions: Vec<Instruction> = Vec::new();
    let some_starting_command: Option<&Token> = tokens.first();
    if let Some(starting_command) = some_starting_command {
        if let TokenType::Command(cmd_type) = &starting_command.token_type {
            instructions = match cmd_type {
                CommandType::CreateTable => parse_create_table(&tokens, cmd_type)?,
                CommandType::Select => parse_select(&tokens)?,
                CommandType::InsertInto => parse_insert_into(&tokens)?,
                CommandType::Update => parse_update(&tokens)?,
                CommandType::Delete => parse_delete(&tokens)?,
            };
            Ok(instructions)
        } else {
            Err(ParsingError::FirstTokenNotCommand {
                found_content: starting_command.content.to_string(),
            })
        }
    } else {
        // In the case of an empty query, an empty Vec of instructions should be returned
        Ok(instructions)
    }
}

fn parse_create_table(tokens: &[Token], cmd_type: &CommandType) -> InstructionResult {
    // At this point, the second token should be the name of the table (else raise a ParsingError)
    if tokens.len() <= 1 {
        return Err(ParsingError::TokenNotFound {
            expected: TokenType::TargetName,
        });
    }
    // After the check above, we can safely make a peekable iterator over tokens, and ignore the
    // first element.
    let mut tokens_iter = tokens[1..].iter().peekable();
    // Here we can also safely unwrap, thanks to the check above
    let table_name = tokens_iter.next().unwrap();
    let some_open_paren = tokens_iter.peek();
    if some_open_paren.is_none() {
        return Err(ParsingError::TokenNotFound {
            expected: TokenType::TargetName,
        });
    }
    if !some_open_paren.unwrap().content.starts_with('(') {
        return Err(ParsingError::MissingCharInToken {
            missing_char: '(',
            char_expected_position: 0,
            found_content: some_open_paren.unwrap().content,
        });
    }
    let found_columns: Vec<Column> = Vec::new();
    let some_end_char_position = tokens_iter.position(|&token| token.content.chars().any(|c| [',', ')'].contains(&c)));
    let all_column_tokens = match some_end_char_position {
        Some(comma_position) => tokens_iter[2..comma_position].collect(),
        None => todo!(),
    }
}

fn parse_select(tokens: &[Token]) -> InstructionResult {}

fn parse_insert_into(tokens: &[Token]) -> InstructionResult {}

fn parse_update(tokens: &[Token]) -> InstructionResult {}

fn parse_delete(tokens: &[Token]) -> InstructionResult {}
