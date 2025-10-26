use crate::tokenizer::{CommandType, Token, TokenType};
use thiserror::Error;

#[derive(Error, Debug)]
enum ParsingError {
    #[error("First token for statement was not a valid command: '{0}'")]
    FirstTokenNotCommand(String),
}

struct Instruction {
    base_command: CommandType,
    target_table: String,
    columns: Vec<String>,
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<Instruction>, ParsingError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let some_starting_command: Option<&Token> = tokens.first();
    if let Some(starting_command) = some_starting_command {
        if let TokenType::Command(cmd_type) = &starting_command.token_type {
            instructions = match cmd_type {
                CommandType::CreateTable => parse_create_table(&tokens),
                CommandType::Select => parse_select(&tokens),
                CommandType::InsertInto => parse_insert_into(&tokens),
                CommandType::Update => parse_update(&tokens),
                CommandType::Delete => parse_delete(&tokens),
            };
            Ok(instructions)
        } else {
            Err(ParsingError::FirstTokenNotCommand(
                starting_command.content.to_string(),
            ))
        }
    } else {
        Ok(instructions)
    }
}

fn parse_create_table(tokens: &[Token]) -> Vec<Instruction> {}

fn parse_select(tokens: &[Token]) -> Vec<Instruction> {}

fn parse_insert_into(tokens: &[Token]) -> Vec<Instruction> {}

fn parse_update(tokens: &[Token]) -> Vec<Instruction> {}

fn parse_delete(tokens: &[Token]) -> Vec<Instruction> {}
