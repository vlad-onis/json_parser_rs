use thiserror::Error;

use super::lexer::*;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f32),
    JsonObject(Vec<(String, JsonValue)>),
    JsonArray(Vec<JsonValue>),
    Boolean(bool),
    Null,
    Empty,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Non matching parantheses")]
    InvalidParantheses,

    #[error("Could not parse json")]
    InvalidJsonFormat,
}

fn valid_parantheses(token_stream: &TokenStream) -> bool {
    let mut parantheses_stack: Vec<Token> = Vec::new();

    // add a parantheses hashmap when you introduce the others.
    for token in token_stream.iter() {
        // for token in token_stream.clone().into_iter() {
        // or together the other types of parantheses here, later
        if *token == constants::LEFT_BRACE.into() {
            parantheses_stack.push(token.to_owned());
        } else {
            match *token {
                Token::JsonCharacter(_) => {
                    let last_inserted = parantheses_stack.pop();
                    if last_inserted.is_none() {
                        return false;
                    }
                    let last_inserted = last_inserted.unwrap();

                    if (*token == constants::RIGHT_BRACE.into())
                        && !(last_inserted == constants::LEFT_BRACE.into())
                    {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
    }
    true
}

pub fn parse(input_stream: TokenStream) -> Result<JsonValue, ParseError> {
    if !valid_parantheses(&input_stream) {
        return Err(ParseError::InvalidParantheses);
    }

    if input_stream.is_empty() {
        return Ok(JsonValue::Empty);
    }

    if input_stream[0] != constants::LEFT_BRACE.into()
        || input_stream[0] != constants::LEFT_BRACKET.into()
    {
        return Err(ParseError::InvalidJsonFormat);
    }

    todo!("Parse object or array");
}

pub fn parse_object(token_stream: &Vec<Token>, current_index: usize) {
    todo!("Parse object members");
}

pub fn parse_object_members(token_stream: &Vec<Token>, current_index: usize) {
    todo!("Parse pair");
}

pub fn parse_pair(token_stream: &Vec<Token>, current_index: usize) {
    todo!("Ensure pair structure and parse value");
}

pub fn parse_value(token_stream: &Vec<Token>, current_index: usize) {
    todo!("determine the value type: String, Number, Boolean, Object, Array, Null");
}

pub fn parse_array(token_stream: &Vec<Token>, current_index: usize) {
    todo!("parse array elements");
}

pub fn parse_array_elements(token_stream: &Vec<Token>, current_index: usize) {
    todo!("Ensure element structure and parse value");
}

#[cfg(test)]
pub mod syntactic_analyzer_tests {
    use super::*;
    use crate::json::lexer::TokenStream;

    #[test]
    pub fn test_parantheses_empty_stream() {
        let token_stream = TokenStream::default();

        assert!(valid_parantheses(&token_stream));
        assert_eq!(parse(token_stream).unwrap(), JsonValue::Empty);
    }

    #[test]
    pub fn test_parantheses_valid() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(constants::RIGHT_BRACE.into());

        assert!(valid_parantheses(&token_stream));
    }

    #[test]
    pub fn test_parantheses_valid_many() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(constants::LEFT_BRACE.into());

        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(constants::RIGHT_BRACE.into());

        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(constants::RIGHT_BRACE.into());

        token_stream.push(constants::RIGHT_BRACE.into());
        token_stream.push(constants::RIGHT_BRACE.into());

        assert!(valid_parantheses(&token_stream));
    }
}
