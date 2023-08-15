use thiserror::Error;

use super::lexer::*;

#[derive(Debug, PartialEq)]
pub struct JsonPair(pub String, pub JsonValue);

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f32),
    JsonObject(Vec<JsonPair>),
    JsonArray(Vec<JsonValue>),
    Boolean(bool),
    Null,
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Non matching parantheses")]
    InvalidParantheses,

    #[error("Could not parse object or array, due to invalid first token")]
    NotValidJsonObjectOrArray,

    #[error("Empty Object")]
    EmptyObject,

    #[error("Could not parse pair")]
    InvalidPair,

    #[error("Could not parse value")]
    InvalidValue,

    #[error("Empty json is invalid")]
    EmptyJson,
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
                Token::JsonCharacter(Character('}')) | Token::JsonCharacter(Character(']')) => {
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

                    if (*token == constants::RIGHT_BRACKET.into())
                        && !(last_inserted == constants::LEFT_BRACKET.into())
                    {
                        return false;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }
    true
}

pub fn parse(mut input_stream: TokenStream) -> Result<JsonValue, ParseError> {
    if !valid_parantheses(&input_stream) {
        return Err(ParseError::InvalidParantheses);
    }

    if input_stream.is_empty() {
        return Err(ParseError::EmptyJson);
    }

    if input_stream[0] != constants::LEFT_BRACE.into()
        && input_stream[0] != constants::LEFT_BRACKET.into()
    {
        return Err(ParseError::NotValidJsonObjectOrArray);
    }

    input_stream.retain(|x| *x != '\n'.into());

    parse_object(&mut input_stream[1..])
}

pub fn parse_object(mut token_stream: &mut [Token]) -> Result<JsonValue, ParseError> {
    let members = parse_object_members(&mut token_stream.to_vec())?;
    Ok(JsonValue::JsonObject(members))
}

pub fn parse_object_members(mut token_stream: &mut [Token]) -> Result<Vec<JsonPair>, ParseError> {
    let mut pair_index = 0;
    let mut result = Vec::new();
    while pair_index < token_stream.len() {
        let pair = parse_pair(&mut token_stream[pair_index..]);

        match pair {
            Ok(pair) => {
                result.push(pair);
                pair_index += 3;
            }
            Err(ParseError::EmptyObject) => {
                token_stream = &mut token_stream[1..];
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(result)
}

pub fn parse_pair(mut token_stream: &mut [Token]) -> Result<JsonPair, ParseError> {
    if token_stream.len() < 3 {
        if token_stream[0] != '}'.into() {
            return Err(ParseError::InvalidPair);
        } else {
            return Err(ParseError::EmptyObject);
        }
    }

    if token_stream[0] == ','.into() {
        token_stream = &mut token_stream[1..];
    }

    let Token::JsonString(name) = token_stream[0].clone() else {
        return Err(ParseError::InvalidPair);
    };

    if token_stream[1] != ':'.into() {
        return Err(ParseError::InvalidPair);
    }

    let value = parse_value(&token_stream[2..])?;
    let result: JsonPair = JsonPair(name, value);
    Ok(result)
}

pub fn parse_value(token_stream: &[Token]) -> Result<JsonValue, ParseError> {
    match &token_stream[0] {
        Token::JsonString(st) => Ok(JsonValue::String(st.to_owned())),
        Token::Number(nr) => Ok(JsonValue::Number(nr.to_owned())),
        Token::Boolean(b) => Ok(JsonValue::Boolean(b.to_owned())),
        Token::JsonCharacter(_) => Err(ParseError::InvalidValue),
        Token::Null => Ok(JsonValue::Null),
        Token::Other(_) => Err(ParseError::InvalidValue),
    }
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
        assert_eq!(parse(token_stream).err(), Some(ParseError::EmptyJson));
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

    #[test]
    pub fn test_parantheses_valid_json() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(Token::JsonString(String::from("key")));
        token_stream.push(':'.into());
        token_stream.push(Token::JsonString(String::from("value")));
        token_stream.push(constants::RIGHT_BRACE.into());

        assert!(valid_parantheses(&token_stream));
    }

    #[test]
    pub fn test_parse_empty_object() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(constants::RIGHT_BRACE.into());

        let res = parse(token_stream).unwrap();
        assert_eq!(res, JsonValue::JsonObject(vec![]));
    }

    #[test]
    pub fn test_parse_1_member_string() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(Token::JsonString(String::from("key")));
        token_stream.push(':'.into());
        token_stream.push(Token::JsonString(String::from("value")));
        token_stream.push(constants::RIGHT_BRACE.into());
        let res = parse(token_stream).unwrap();
        assert_eq!(
            res,
            JsonValue::JsonObject(vec![JsonPair(
                "key".to_string(),
                JsonValue::String("value".to_string())
            )])
        );
    }

    #[test]
    pub fn test_parse_2_member_string_and_number() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(Token::JsonString(String::from("key1")));
        token_stream.push(':'.into());
        token_stream.push(Token::JsonString(String::from("value")));
        token_stream.push(Token::JsonString(String::from("key2")));
        token_stream.push(':'.into());
        token_stream.push(Token::Number(1.0));
        token_stream.push(constants::RIGHT_BRACE.into());
        let res = parse(token_stream).unwrap();
        assert_eq!(
            res,
            JsonValue::JsonObject(vec![
                JsonPair("key1".to_string(), JsonValue::String("value".to_string())),
                JsonPair("key2".to_string(), JsonValue::Number(1.0))
            ])
        );
    }

    #[test]
    pub fn test_parse_all_members() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(Token::JsonString(String::from("key1")));
        token_stream.push(':'.into());
        token_stream.push(Token::JsonString(String::from("value")));
        token_stream.push(Token::JsonString(String::from("key2")));
        token_stream.push(':'.into());
        token_stream.push(Token::Number(1.0));
        token_stream.push(Token::JsonString(String::from("key3")));
        token_stream.push(':'.into());
        token_stream.push(Token::Boolean(true));
        token_stream.push(Token::JsonString(String::from("key4")));
        token_stream.push(':'.into());
        token_stream.push(Token::Null);
        token_stream.push(constants::RIGHT_BRACE.into());
        let res = parse(token_stream).unwrap();
        assert_eq!(
            res,
            JsonValue::JsonObject(vec![
                JsonPair("key1".to_string(), JsonValue::String("value".to_string())),
                JsonPair("key2".to_string(), JsonValue::Number(1.0)),
                JsonPair("key3".to_string(), JsonValue::Boolean(true),),
                JsonPair("key4".to_string(), JsonValue::Null,)
            ])
        );
    }
}
