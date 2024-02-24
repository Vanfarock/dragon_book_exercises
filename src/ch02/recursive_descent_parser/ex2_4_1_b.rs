#[derive(Debug, PartialEq)]
enum Token {
    Epsilon,
    OpenParam,
    CloseParam,
    Invalid,
}

pub struct ParserB {
    input: Vec<char>,
    lookahead_index: usize,
}

impl ParserB {
    // Recursive-descent parser for the following grammar:
    // S := S(S)S | ε
    // Since it is left-recursive, it may lead to infinite loop
    // when using a recursive-descent parser
    // To solve this, it is necessary to convert to:
    // S := R
    // R := (S)S | ε

    pub fn new(input: &str) -> Self {
        ParserB {
            input: input.chars().collect(),
            lookahead_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.internal_parse()?;

        if self.lookahead_index == self.input.len() {
            Ok(())
        } else {
            Err("Syntax error: Invalid input".to_string())
        }
    }

    fn internal_parse(&mut self) -> Result<(), String> {
        match self.next_token() {
            Token::OpenParam => {
                self.internal_parse()?;
                self.match_token(Token::CloseParam)?;
                self.internal_parse()?;
                Ok(())
            }
            Token::CloseParam => {
                self.lookahead_index -= 1;
                Ok(())
            }
            Token::Epsilon => Ok(()),
            Token::Invalid => Err("Syntax error: Invalid char".to_string()),
        }
    }

    fn match_token(&mut self, expected: Token) -> Result<(), String> {
        if self.next_token() == expected {
            Ok(())
        } else {
            Err(format!(
                "Syntax error: Expected {:?} at position {}",
                expected, self.lookahead_index
            ))
        }
    }

    fn next_token(&mut self) -> Token {
        if self.lookahead_index >= self.input.len() {
            return Token::Epsilon;
        }

        match self.input[self.lookahead_index] {
            '(' => {
                self.lookahead_index += 1;
                Token::OpenParam
            }

            ')' => {
                self.lookahead_index += 1;
                Token::CloseParam
            }
            _ => Token::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("()")]
    #[case("()()()")]
    #[case("(()())()(())")]
    #[case("(())()")]
    #[case("(())(())(())")]
    fn test_2_4_1_b_valid(#[case] input: &str) -> Result<(), String> {
        assert_eq!(ParserB::new(input).parse()?, ());
        Ok(())
    }

    #[rstest]
    #[case("a", "Syntax error: Invalid char")]
    #[case("(", "Syntax error: Expected CloseParam at position 1")]
    #[case(")", "Syntax error: Invalid input")]
    #[case(")(", "Syntax error: Invalid input")]
    #[case("())", "Syntax error: Invalid input")]
    #[case("(()", "Syntax error: Expected CloseParam at position 3")]
    // #[case("()())", "Syntax error: Invalid input")]
    fn test_2_4_1_invalid(#[case] input: &str, #[case] error_message: &str) -> Result<(), String> {
        assert_eq!(
            ParserB::new(input).parse().unwrap_err(),
            error_message.to_string()
        );
        Ok(())
    }
}
