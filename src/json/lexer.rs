use std::num::ParseFloatError;

use thiserror::Error;

pub enum JsonValues {
    Quote,
}

impl JsonValues {
    pub fn as_char(value: JsonValues) -> char {
        match value {
            JsonValues::Quote => '"',
        }
    }
}

#[derive(Debug, Error)]
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
}

// (The content between quotes, the index of the next char after the quote)
pub type LexStringOutput = (String, usize);

pub type LexNumberOutput = (f32, usize);

pub type LexBoolOutput = (bool, usize);

fn crop_content(json_content: &str, index: usize) -> &str {
    if index == json_content.len() - 1 {
        ""
    } else {
        &json_content[(index + 1)..]
    }
}

pub fn lex_bool(json_content: &str) -> Result<LexBoolOutput, LexerError> {
    if json_content.is_empty() {
        return Err(LexerError::EmptyInput);
    }

    let mut result: bool = true;
    let mut last_processed_index: usize = 0;
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
        } else if ch == '\n' {
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

    if json_content[0] != JsonValues::as_char(JsonValues::Quote) {
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

pub fn lex(json_content: &str) -> Vec<String> {
    let mut json_content = json_content.trim();
    let mut tokens: Vec<String> = Vec::new();

    loop {
        if json_content.is_empty() {
            break;
        }

        let lex_string_result = lex_string(json_content);
        if let Ok((accumulated_string, last_processed_index)) = lex_string_result {
            tokens.push(accumulated_string);
            json_content = crop_content(json_content, last_processed_index);
        }

        let lex_number_result = lex_number(json_content);
        if let Ok((number, last_processed_index)) = lex_number_result {
            tokens.push(number.to_string());
            json_content = crop_content(json_content, last_processed_index);
        }

        if let Ok((result, last_processed_index)) = lex_bool(json_content) {
            tokens.push(result.to_string());
            json_content = crop_content(json_content, last_processed_index);
        } else {
            // the following snippet just adds any character that is not a string to
            // the token list.
            // TODO: This should be part of the parsing itself.

            let first = json_content.chars().next().unwrap();
            if first != ' ' && first != '\n' {
                tokens.push(first.to_string());
            }

            json_content = crop_content(json_content, 0);
        }
    }

    tokens
}

#[cfg(test)]
pub mod lexer_tests {
    use super::*;

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

        let res = lex(json);
        assert_eq!(res, vec![String::from("{"), String::from("}")]);
    }

    #[test]
    pub fn test_lexing_only_strings_valid_json() {
        let json = r#"
{
    "key":"value"
}
"#;
        let json = json.trim();

        let res = lex(json);
        let expected = vec![
            String::from("{"),
            String::from("key"),
            String::from(":"),
            String::from("value"),
            String::from("}"),
        ];

        assert_eq!(res, expected);
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
    pub fn test_lext_json_containing_number() {
        let json = r#"
{
    "key":42
}
"#;
        let json = json.trim();

        let res = lex(json);
        let expected = vec![
            String::from("{"),
            String::from("key"),
            String::from(":"),
            String::from("42"),
            String::from("}"),
        ];

        assert_eq!(res, expected);
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
    pub fn test_lext_json_containing_bool() {
        let json = r#"
{
    "key":true
}
"#;
        let json = json.trim();

        let res = lex(json);
        let expected = vec![
            String::from("{"),
            String::from("key"),
            String::from(":"),
            String::from("true"),
            String::from("}"),
        ];

        assert_eq!(res, expected);
    }
}
