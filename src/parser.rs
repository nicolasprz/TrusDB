use crate::lookahead::LookaheadExt;
use crate::tokenizer::{CommandType, Token, TokenType};
use thiserror::Error;

// TODO list (general for this script):
//   - Find a way of parsing data types with multiple words (review Tokenizer ?)
//   - Find a way of handling varchar data type

// TODO: divide this enum into multiple ones
#[derive(Error, Debug)]
pub enum ParsingError {
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
    #[error("Unexpected '{unexpected_char}' found at position {char_position} in '{content}'")]
    UnexpectedCharInToken {
        unexpected_char: char,
        char_position: usize,
        content: String,
    },
    #[error("Tokens to parse a column were found empty")]
    EmptyColumnTokens,
    #[error("No data type was provided for column {column_name}")]
    NoDataTypeProvided { column_name: String },
    #[error("Unexpected data type '{found}'")]
    UnexpectedDataTypeProvided { found: String },
    #[error("Expected '{missing_char}' at the end of statement")]
    MissingEndOfStatementChar { missing_char: char },
}

type InstructionResult = Result<Option<Instruction>, ParsingError>;

#[derive(Debug)]
pub struct Instruction {
    base_command: CommandType,
    target_table: String,
    columns: Vec<Column>,
}

#[derive(Debug)]
enum DataType {
    Float,
    Integer,
    Text,
    Bool,
    Uuid,
}

impl DataType {
    fn from_string(data_type: String) -> Result<DataType, ParsingError> {
        match data_type.to_lowercase().as_str() {
            "float" => Ok(DataType::Float),
            "integer" => Ok(DataType::Integer),
            "text" => Ok(DataType::Text),
            "bool" => Ok(DataType::Bool),
            "uuid" => Ok(DataType::Uuid),
            &_ => Err(ParsingError::UnexpectedDataTypeProvided { found: data_type }),
        }
    }
}

#[derive(Debug)]
struct Column {
    name: String,
    data_type: DataType,
    values: Vec<DataType>,
    is_primary_key: bool,
}

const CHARACTERS_TO_CLEAN: [char; 3] = [',', ')', ';'];

pub fn parse_tokens(tokens: Vec<Token>) -> InstructionResult {
    let some_starting_command: Option<&Token> = tokens.first();
    if let Some(starting_command) = some_starting_command {
        if let TokenType::Command(cmd_type) = &starting_command.token_type {
            let instruction = match cmd_type {
                CommandType::CreateTable => parse_create_table(&tokens, cmd_type)?,
                CommandType::Select => parse_select(&tokens)?,
                CommandType::InsertInto => parse_insert_into(&tokens)?,
                CommandType::Update => parse_update(&tokens)?,
                CommandType::Delete => parse_delete(&tokens)?,
            };
            Ok(instruction)
        } else {
            Err(ParsingError::FirstTokenNotCommand {
                found_content: starting_command.content.to_string(),
            })
        }
    } else {
        // In the case of an empty query, an empty Vec of instructions should be returned
        Ok(None)
    }
}

fn parse_create_table(tokens: &[Token], cmd_type: &CommandType) -> InstructionResult {
    // At this point, the second token should be the name of the table (else raise a ParsingError)
    if tokens.len() <= 1 {
        return Err(ParsingError::TokenNotFound {
            expected: TokenType::Expression,
        });
    }
    // After the check above, we can safely make a peekable iterator over tokens, and ignore the
    // first element.
    let mut tokens_iter = tokens[1..].into_iter().peekable();
    // Here we can also safely unwrap, thanks to the check above
    let table_name = tokens_iter.next().unwrap();
    if let Some(open_paren_position) = table_name.content.chars().position(|c| c == '(') {
        return Err(ParsingError::UnexpectedCharInToken {
            unexpected_char: '(',
            char_position: open_paren_position,
            content: table_name.content.to_string(),
        });
    }
    {
        let some_open_paren = tokens_iter.peek();
        if some_open_paren.is_none() {
            return Err(ParsingError::TokenNotFound {
                expected: TokenType::Expression,
            });
        }
        if !&some_open_paren.unwrap().content.starts_with('(') {
            return Err(ParsingError::MissingCharInToken {
                missing_char: '(',
                char_expected_position: 0,
                found_content: some_open_paren.unwrap().content.to_string(),
            });
        }
    }
    let mut found_columns: Vec<Column> = Vec::new();
    while tokens_iter.peek().is_some() {
        let mut cloned_iter = tokens_iter.clone();
        let some_end_char_position: Option<usize> = cloned_iter
            .clone()
            .position(|token| token.content.chars().any(|c| [',', ')'].contains(&c)));
        if let Some(end_pos) = some_end_char_position {
            let column_tokens: Vec<&Token> = cloned_iter.by_ref().take(end_pos + 1).collect();
            found_columns.push(parse_column(column_tokens)?);
            tokens_iter = cloned_iter;
        }
    }
    if let Some(last_token) = tokens_iter.last() {
        if !last_token.content.contains(')') {
            let expected_closing_paren_position: Option<usize> = last_token
                .content
                .char_indices()
                .find(|&(_, c)| c.is_ascii_alphabetic())
                .map(|(i, _)| i);
            return Err(ParsingError::MissingCharInToken {
                missing_char: ')',
                char_expected_position: expected_closing_paren_position.unwrap(),
                found_content: last_token.content.to_string(),
            });
        }
        if !last_token.content.ends_with(';') {
            return Err(ParsingError::MissingEndOfStatementChar { missing_char: ';' });
        }
    }
    Ok(Some(Instruction {
        base_command: cmd_type.clone(),
        target_table: table_name.content.to_string(),
        columns: found_columns,
    }))
}

fn parse_column(column_tokens: Vec<&Token>) -> Result<Column, ParsingError> {
    if column_tokens.len() == 0 {
        return Err(ParsingError::EmptyColumnTokens);
    }
    let mut column_tokens_iter = column_tokens.into_iter().lookahead();
    let column_name: String = column_tokens_iter
        .next()
        .unwrap()
        .content
        .chars()
        .filter(|&c| c != '(')
        .collect();
    let column_type: DataType = match column_tokens_iter.next() {
        None => Err(ParsingError::NoDataTypeProvided {
            column_name: column_name.clone(),
        }),
        Some(token) => DataType::from_string(
            token
                .content
                .chars()
                .filter(|c| ![',', ')', ';'].contains(c))
                .collect(),
        ),
    }?;
    // By default, this value is set to false, unless a primary key token is found
    let mut is_primary_key = false;
    println!("{:?}", column_tokens_iter.peek(2));
    if column_tokens_iter.peek(2).is_some() {
        let result: String = column_tokens_iter
            .take(2)
            .map(|t| t.content.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Primary key content: {result:?}");
        let clean_result: String = result
            .to_lowercase()
            .chars()
            .filter(|c| !CHARACTERS_TO_CLEAN.contains(c))
            .collect();
        is_primary_key = clean_result == "primary key";
    }
    Ok(Column {
        name: column_name,
        data_type: column_type,
        values: Vec::new(),
        is_primary_key,
    })
}

fn parse_select(tokens: &[Token]) -> InstructionResult {
    todo!("Not implemented");
    Ok(None)
}

fn parse_insert_into(tokens: &[Token]) -> InstructionResult {
    todo!("Not implemented");
    Ok(None)
}

fn parse_update(tokens: &[Token]) -> InstructionResult {
    todo!("Not implemented");
    Ok(None)
}

fn parse_delete(tokens: &[Token]) -> InstructionResult {
    todo!("Not implemented");
    Ok(None)
}
