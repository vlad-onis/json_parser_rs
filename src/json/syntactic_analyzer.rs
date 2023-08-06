use super::lexer::*;

fn valid_parantheses(token_stream: TokenStream) -> bool {
    let mut parantheses_stack: Vec<Token> = Vec::new();

    // add a parantheses hashmap when you introduce the others.

    for token in token_stream.clone().into_iter() {
        // or together the other types of parantheses here, later
        if token == constants::LEFT_BRACE.into() {
            parantheses_stack.push(token);
        } else {
            match token {
                Token::JsonCharacter(_) => {
                    let last_inserted = parantheses_stack.pop();
                    if last_inserted.is_none() {
                        return false;
                    }
                    let last_inserted = last_inserted.unwrap();

                    if (token == constants::RIGHT_BRACE.into())
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

#[cfg(test)]
pub mod syntactic_analyzer_tests {
    use super::*;
    use crate::json::lexer::TokenStream;

    #[test]
    pub fn test_parantheses_empty_stream() {
        let mut token_stream = TokenStream::default();

        assert!(valid_parantheses(token_stream));
    }

    #[test]
    pub fn test_parantheses_valid() {
        let mut token_stream = TokenStream::default();
        token_stream.push(constants::LEFT_BRACE.into());
        token_stream.push(constants::RIGHT_BRACE.into());

        assert!(valid_parantheses(token_stream));
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

        assert!(valid_parantheses(token_stream));
    }
}
