#[cfg(test)]
pub mod integration_tests {

    use std::path::PathBuf;

    use json_parser_rs::json::{
        lexer::{self, lex, LexerError},
        syntactic_analyzer::{self, parse, JsonPair, JsonValue, ParseError},
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

    #[test]
    pub fn integration_test_step3() {
        let file = PathBuf::from("tests/step3/invalid.json");
        assert!(file.is_file());
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content);
        assert_eq!(token_stream.err(), Some(LexerError::InvalidJson));

        let file = PathBuf::from("tests/step3/valid.json");
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content).unwrap();
        let value = syntactic_analyzer::parse(token_stream).unwrap();

        assert_eq!(
            value,
            JsonValue::JsonObject(vec![
                JsonPair(String::from("key1"), JsonValue::Boolean(true)),
                JsonPair(String::from("key2"), JsonValue::Boolean(false)),
                JsonPair(String::from("key3"), JsonValue::Null,),
                JsonPair(
                    String::from("key4"),
                    JsonValue::String(String::from("value"))
                ),
                JsonPair(String::from("key5"), JsonValue::Number(101.0)),
            ])
        );
    }

    #[test]
    pub fn integration_test_step4() {
        let file = PathBuf::from("tests/step4/invalid.json");
        assert!(file.is_file());
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content);
        assert_eq!(token_stream.err(), Some(LexerError::InvalidJson));

        let file = PathBuf::from("tests/step4/valid.json");
        assert!(file.is_file());
        let content = std::fs::read_to_string(file).unwrap();
        let token_stream = lex(&content).unwrap();
        let value = syntactic_analyzer::parse(token_stream);

        // Inner object or arrays are not yet supported
        assert_eq!(value.err(), Some(ParseError::InvalidValue));
    }
}
