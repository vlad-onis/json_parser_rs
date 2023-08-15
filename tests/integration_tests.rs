#[cfg(test)]
pub mod integration_tests {

    use std::path::PathBuf;

    use json_parser_rs::json::{
        lexer::{self, lex, LexerError},
        syntactic_analyzer::{self, JsonPair, JsonValue, ParseError},
    };

    #[test]
    pub fn integration_test_step1() {
        let file = PathBuf::from("tests/step1/invalid.json");
        assert!(file.is_file());
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content);
        assert_eq!(token_stream.err(), Some(LexerError::EmptyInput));

        let file = PathBuf::from("tests/step1/valid.json");
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content).unwrap();
        let value = syntactic_analyzer::parse(token_stream).unwrap();

        assert_eq!(value, JsonValue::JsonObject(vec![]));
    }

    #[test]
    pub fn integration_test_step2() {
        let file = PathBuf::from("tests/step2/invalid.json");
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content);
        assert_eq!(token_stream.err(), Some(LexerError::EndingInComma));

        let file = PathBuf::from("tests/step2/invalid2.json");
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content);

        assert_eq!(token_stream.err(), Some(lexer::LexerError::InvalidJson));

        let file = PathBuf::from("tests/step2/valid.json");
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content).unwrap();
        let value = syntactic_analyzer::parse(token_stream).unwrap();
        assert_eq!(
            value,
            JsonValue::JsonObject(vec![JsonPair(
                String::from("key"),
                JsonValue::String(String::from("value"))
            )])
        );

        let file = PathBuf::from("tests/step2/valid2.json");
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content).unwrap();
        let value = syntactic_analyzer::parse(token_stream).unwrap();
        assert_eq!(
            value,
            JsonValue::JsonObject(vec![
                JsonPair(
                    String::from("key"),
                    JsonValue::String(String::from("value"))
                ),
                JsonPair(
                    String::from("key2"),
                    JsonValue::String(String::from("value"))
                ),
            ])
        );
    }
}
