use crate::prompts;
use regex::Regex;
use std::io;
use std::io::Write;
use thiserror::Error;

const EXPRESSION_PATTERN: &str = r"(?i)^(?:'(?:[^']*)'|-?\d+(?:\.\d+)?|true|false)$";

fn make_expression_regex() -> Regex {
    Regex::new(EXPRESSION_PATTERN).expect("Error creating regex for expressions")
}

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

// enum Data {
//     Float(f32),
//     Integer(i32),
//     Text(String),
//     Bool(bool),
// }

enum TokenType {
    Command(CommandType),
    Operator(OperatorType),
    ColumnName(String),
    Expression(String),
}

enum OperatorType {
    Equal,
    Plus,
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

fn tokenize_user_input(user_input: &str) -> Result<Vec<Token>, TokenizingError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut word_iter = user_input.split_whitespace().peekable();
    let expression_regex: Regex = make_expression_regex();
    while let Some(word) = word_iter.next() {
        if let Some(&next) = word_iter.peek() {
            let current_token = build_token(word, next, &expression_regex)?;
            tokens.push(current_token);
        }
    }
    Ok(tokens)
}

fn build_token(word: &str, next: &str, expression_regex: &Regex) -> Result<Token, TokenizingError> {
    let owned_word: String = word.to_string();
    match word.to_lowercase().as_str() {
        // Commands
        "create" => generate_multiple_words_token(
            TokenType::Command(CommandType::CreateTable),
            word,
            next,
            "table",
        ),
        "select" => Ok(Token::new(
            TokenType::Command(CommandType::Select),
            owned_word,
        )),
        "insert" => generate_multiple_words_token(
            TokenType::Command(CommandType::InsertInto),
            word,
            next,
            "into",
        ),
        "update" => Ok(Token::new(
            TokenType::Command(CommandType::Update),
            owned_word,
        )),
        "delete" => Ok(Token::new(
            TokenType::Command(CommandType::Delete),
            owned_word,
        )),
        // Operators
        "=" => Ok(Token::new(
            TokenType::Operator(OperatorType::Equal),
            owned_word,
        )),
        "+" => Ok(Token::new(
            TokenType::Operator(OperatorType::Equal),
            owned_word,
        )),
        // Expressions and column names
        &_ => if expression_regex.is_match(word) {
            Ok(Token::new(TokenType::Expression(owned_word.clone()), owned_word))
        } else {
            Ok(Token::new(TokenType::ColumnName(owned_word.clone()), owned_word))
        },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_patterns() {
        let re = make_expression_regex();
        let valid_cases = [
            "'hello'", "'123abc'", "''", "42", "-15", "3.1415", "-0.5", "true", "False", "TRUE",
        ];

        for case in valid_cases {
            assert!(re.is_match(case), "Failed: regex should match `{}`", case);
        }
    }

    #[test]
    fn test_invalid_patterns() {
        let re = make_expression_regex();
        let invalid_cases = [
            "\"hello\"", // double quotes
            "'unclosed", // missing closing quote
            "3.14.15",   // invalid number
            "tru",       // partial boolean
            "yes",       // not a boolean
            "2.",        // no digits after dot
            ".5",        // no digits before dot
            "falsehood", // longer word than "false"
        ];

        for case in invalid_cases {
            assert!(
                !re.is_match(case),
                "Failed: regex should NOT match `{}`",
                case
            );
        }
    }
}
