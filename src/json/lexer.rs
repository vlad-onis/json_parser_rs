use std::{num::ParseFloatError, ops::Deref, ops::DerefMut};

use thiserror::Error;

pub mod constants {
    pub const QUOTE: char = '"';
    pub const LEFT_BRACKET: char = '[';
    pub const RIGHT_BRACKET: char = ']';
    pub const LEFT_BRACE: char = '{';
    pub const RIGHT_BRACE: char = '}';
    pub const COLUMN: char = ':';
    pub const NEW_LINE: char = '\n';
    pub const COMMA: char = ',';
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Character(pub char);

impl Character {
    pub fn new(ch: char) -> Result<Character, LexerError> {
        TryFrom::try_from(ch)
    }
}

impl TryFrom<char> for Character {
    type Error = LexerError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if ch == constants::QUOTE
            || ch == constants::LEFT_BRACKET
            || ch == constants::RIGHT_BRACKET
            || ch == constants::LEFT_BRACE
            || ch == constants::RIGHT_BRACE
            || ch == constants::COLUMN
            || ch == constants::NEW_LINE
            || ch == constants::COMMA
        {
            return Ok(Character(ch));
        }

        Err(LexerError::NotAJsonChar)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    JsonString(String),
    Number(f32),
    Boolean(bool),
    JsonCharacter(Character),
    Null,
    // this one exists for testing purposes only
    Other(char),
}

impl From<char> for Token {
    fn from(ch: char) -> Self {
        match Character::try_from(ch) {
            Ok(character_token) => Token::JsonCharacter(character_token),
            Err(_e) => Token::Other(ch),
        }
    }
}

#[derive(Debug, Default)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum LexerError {
    #[error("No input was provided")]
    EmptyInput,

    #[error("Could not lex string")]
    NotAString,

    #[error("Could not lex number")]
    NotANumber,

    #[error("Parsing number failed due to: {0}")]
    NotAFloat(ParseFloatError),

    #[error("Could not lex boolean")]
    NotABool,

    #[error("Character is a not a specific Json character")]
    NotAJsonChar,

    #[error("Could not lex Null")]
    NotANull,

    #[error("Failed to lex the input json")]
    InvalidJson,
}

// (The content between quotes, the index of the next char after the quote)
pub type LexStringOutput = (String, usize);

pub type LexNumberOutput = (f32, usize);

pub type LexBoolOutput = (bool, usize);

pub type LexNullOutput = (Token, usize);

fn crop_content(json_content: &str, index: usize) -> &str {
    if index == json_content.len() - 1 {
        ""
    } else {
        &json_content[(index + 1)..]
    }
}

pub fn lex_null(json_content: &str) -> Result<LexNullOutput, LexerError> {
    if json_content.is_empty() {
        return Err(LexerError::EmptyInput);
    }

    let result: Token;
    let last_processed_index: usize;
    if json_content.starts_with("null\n") || json_content.starts_with("null,") {
        result = Token::Null;
        last_processed_index = 3; // directly the end of the "null" word
    } else {
        return Err(LexerError::NotANull);
    }

    Ok((result, last_processed_index))
}

pub fn lex_bool(json_content: &str) -> Result<LexBoolOutput, LexerError> {
    if json_content.is_empty() {
        return Err(LexerError::EmptyInput);
    }

    let result: bool;
    let last_processed_index: usize;
    if json_content.starts_with("true") {
        result = true;
        last_processed_index = 3;
    } else if json_content.starts_with("false") {
        result = false;
        last_processed_index = 4;
    } else {
        return Err(LexerError::NotABool);
    }

    Ok((result, last_processed_index))
}

pub fn lex_number(json_content: &str) -> Result<LexNumberOutput, LexerError> {
    if json_content.is_empty() {
        return Err(LexerError::EmptyInput);
    }

    // This can't fail
    let negative = json_content.starts_with('-');
    let skip = if negative { 1 } else { 0 };
    // If the number is negative skip the sign
    let json_content: Vec<char> = json_content.chars().collect();

    let mut number_result = String::new();

    // skip is either 1 or 0,
    // we define the starting point according to that
    let mut last_processed_index = skip;

    for (index, ch) in json_content.into_iter().enumerate().skip(skip) {
        if ch.is_ascii_digit() || ch == '.' {
            number_result.push(ch);
            last_processed_index = index;
        } else if ch == '\n' || ch == ',' {
            break;
        } else {
            return Err(LexerError::NotANumber);
        }
    }

    let number_result = number_result
        .parse::<f32>()
        .map_err(LexerError::NotAFloat)?;
    let number_result = if negative {
        number_result * -1.00
    } else {
        number_result
    };

    Ok((number_result, last_processed_index))
}

pub fn lex_string(json_content: &str) -> Result<LexStringOutput, LexerError> {
    if json_content.is_empty() {
        return Err(LexerError::EmptyInput);
    }

    let json_content: Vec<char> = json_content.chars().collect();

    if json_content[0] != constants::QUOTE {
        return Err(LexerError::NotAString);
    }

    // prepare the result
    let mut accumulated_string = String::new();
    let mut last_processed_character = 0;

    // we are skipping one because we verified the first one to be Quote a few lines above
    // We don't want to accumulate the quote in our final result
    for (index, character) in json_content.into_iter().enumerate().skip(1) {
        match character {
            // can't call as_char in patterns :(
            '"' => {
                last_processed_character = index;
                break;
            }

            ch => {
                accumulated_string.push(ch);
            }
        }
    }

    Ok((accumulated_string, last_processed_character))
}

pub fn lex_character(json_content: &str) -> Result<Token, LexerError> {
    if json_content.is_empty() {
        return Err(LexerError::EmptyInput);
    }

    Ok(json_content.chars().next().unwrap().into())
}

pub fn lex(json_content: &str) -> Result<TokenStream, LexerError> {
    let mut json_content = json_content.trim();
    let mut tokens = TokenStream::default();

    loop {
        if json_content.is_empty() {
            break;
        }

        let lex_string_result = lex_string(json_content);
        if let Ok((accumulated_string, last_processed_index)) = lex_string_result {
            tokens.push(Token::JsonString(accumulated_string));
            json_content = crop_content(json_content, last_processed_index);
        }

        let lex_number_result = lex_number(json_content);
        if let Ok((number, last_processed_index)) = lex_number_result {
            tokens.push(Token::Number(number));
            json_content = crop_content(json_content, last_processed_index);
        }

        if let Ok((result, last_processed_index)) = lex_bool(json_content) {
            tokens.push(Token::Boolean(result));
            json_content = crop_content(json_content, last_processed_index);
        }

        if let Ok((result, last_processed_index)) = lex_null(json_content) {
            tokens.push(result);
            json_content = crop_content(json_content, last_processed_index);
        }

        if let Ok(result) = lex_character(json_content) {
            match result {
                Token::JsonCharacter(_) => {
                    tokens.push(result);
                }
                Token::Other(ch) => {
                    if ch == ' ' {
                        json_content = crop_content(json_content, 0);
                        continue;
                    } else {
                        return Err(LexerError::InvalidJson);
                    }
                }
                _ => {
                    return Err(LexerError::InvalidJson);
                }
            }

            json_content = crop_content(json_content, 0); // crop at 0 because we parsed only 1 character
        } else {
            // the following snippet just adds any character that is not a string to
            // the token list.
            // TODO: This should be part of the parsing itself.

            // let first = json_content.chars().next().unwrap();
            // if first != ' ' && first != '\n' {
            //     tokens.push(first.into());
            // }

            // json_content = crop_content(json_content, 0);

            return Err(LexerError::InvalidJson);
        }
    }

    Ok(tokens)
}

#[cfg(test)]
pub mod lexer_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_lex_string() {
        let string = r#"
            "key""value"
        "#;
        let string = string.trim();

        let (accumulated, last_processed_index) = lex_string(&string).unwrap();

        assert_eq!(accumulated, String::from("key"));
        assert_eq!(last_processed_index, 4);

        let cropped_input = if last_processed_index == string.len() - 1 {
            ""
        } else {
            &string[(last_processed_index + 1)..]
        };
        let (accumulated, last_processed_index) = lex_string(&cropped_input).unwrap();
        assert_eq!(accumulated, String::from("value"));
        assert_eq!(last_processed_index, 6);
    }

    #[test]
    pub fn test_lexing_empty_valid_json() {
        let json = r#"
        {
        }
        "#;
        let json = json.trim();

        let res = lex(json).unwrap();
        let expected = vec![
            '{'.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            '}'.try_into().unwrap(),
        ];
        assert_eq!(*res, expected);
    }

    #[test]
    pub fn test_lexing_only_strings_valid_json() {
        let json = r#"
{
    "key":"value"
}
"#;
        let json = json.trim();

        let res = lex(json).unwrap();

        let expected = vec![
            '{'.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            Token::JsonString("key".to_string()),
            ':'.try_into().unwrap(),
            Token::JsonString("value".to_string()),
            '\n'.try_into().unwrap(),
            '}'.try_into().unwrap(),
        ];

        assert_eq!(*res, expected);
    }

    #[test]
    pub fn test_lex_valid_number() {
        let input = "123";
        let (number, last_processed_index) = lex_number(input).unwrap();

        assert_eq!(number, 123.00);
        assert_eq!(last_processed_index, 2);
    }

    #[test]
    pub fn test_lex_valid_geative_number() {
        let input = "-123";
        let (number, last_processed_index) = lex_number(input).unwrap();

        assert_eq!(number, -123.00);
        assert_eq!(last_processed_index, 3);
    }

    #[test]
    pub fn test_lex_all_zeros() {
        let input = "0000";
        let (number, last_processed_index) = lex_number(input).unwrap();

        assert_eq!(number, 0.00);
        assert_eq!(last_processed_index, 3);
    }

    #[test]
    pub fn test_lex_float_number() {
        let input = "123.4";
        let (number, last_processed_index) = lex_number(input).unwrap();

        assert_eq!(number, 123.40);
        assert_eq!(last_processed_index, 4);
    }

    #[test]
    pub fn test_lex_negative_float_number() {
        let input = "-123.4";
        let (number, last_processed_index) = lex_number(input).unwrap();

        assert_eq!(number, -123.40);
        assert_eq!(last_processed_index, 5);
    }

    #[test]
    pub fn test_lex_json_containing_number() {
        let json = r#"
{
    "key":42
}
"#;
        let json = json.trim();

        let res = lex(json).unwrap();

        let expected = vec![
            '{'.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            Token::JsonString("key".to_string()),
            ':'.try_into().unwrap(),
            Token::Number(42.0),
            '\n'.try_into().unwrap(),
            '}'.try_into().unwrap(),
        ];

        assert_eq!(*res, expected);
    }

    #[test]
    pub fn test_lex_bool_true() {
        let input: &str = "true";
        let (result, last_processed_index) = lex_bool(input).unwrap();

        assert_eq!(result, true);
        assert_eq!(last_processed_index, 3);
    }

    #[test]
    pub fn test_lex_bool_false() {
        let input: &str = "false";
        let (result, last_processed_index) = lex_bool(input).unwrap();

        assert_eq!(result, false);
        assert_eq!(last_processed_index, 4);
    }

    #[test]
    pub fn test_lex_null() {
        let input: &str = "null,";
        let (result, last_processed_index) = lex_null(input).unwrap();

        assert_eq!(result, Token::Null);
        assert_eq!(last_processed_index, 3);
    }

    #[test]
    pub fn test_lex_null_error() {
        let input: &str = "null";
        let result = lex_null(input);

        assert_eq!(result, Err(LexerError::NotANull));
    }

    #[test]
    pub fn test_lex_json_containing_bool() {
        let json = r#"
{
    "key":true
}
"#;
        let json = json.trim();

        let res = lex(json).unwrap();
        let expected = vec![
            '{'.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            Token::JsonString("key".to_string()),
            ':'.try_into().unwrap(),
            Token::Boolean(true),
            '\n'.try_into().unwrap(),
            '}'.try_into().unwrap(),
        ];

        assert_eq!(*res, expected);
    }

    #[test]
    pub fn test_lex_json_containing_all() {
        let json = r#"
{
    "key1":"string",
    "key2":42,
    "key3":true,
    "key4":null
}
"#;
        let json = json.trim();

        let res = lex(json).unwrap();
        let expected = vec![
            '{'.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            Token::JsonString("key1".to_string()),
            ':'.try_into().unwrap(),
            Token::JsonString("string".to_string()),
            ','.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            Token::JsonString("key2".to_string()),
            ':'.try_into().unwrap(),
            Token::Number(42.0),
            ','.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            Token::JsonString("key3".to_string()),
            ':'.try_into().unwrap(),
            Token::Boolean(true),
            ','.try_into().unwrap(),
            '\n'.try_into().unwrap(),
            Token::JsonString("key4".to_string()),
            ':'.try_into().unwrap(),
            Token::Null,
            '\n'.try_into().unwrap(),
            '}'.try_into().unwrap(),
        ];
        assert_eq!(*res, expected);
    }
}
