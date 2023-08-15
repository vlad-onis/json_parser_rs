pub mod json;

use std::path::PathBuf;

use json::lexer;
use json::syntactic_analyzer;

use json::syntactic_analyzer::JsonValue;
use json::syntactic_analyzer::ParseError;

pub fn parse_json_file(path: &PathBuf) -> Result<JsonValue, ParseError> {
    // todo: Fix the unwrap
    let content = std::fs::read_to_string(path).unwrap();
    let token_stream = lexer::lex(&content).unwrap();

    syntactic_analyzer::parse(token_stream)
}
