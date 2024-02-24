#[derive(Debug, PartialEq)]
enum Token {
    Zero,
    One,
    Epsilon,
    Invalid,
}

pub struct ParserC {
    input: Vec<char>,
    lookahead_index: usize,
}

impl ParserC {
    // Recursive-descent parser for the following grammar:
    // S := 0S1 | 01

    pub fn new(input: &str) -> Self {
        ParserC {
            input: input.chars().collect(),
            lookahead_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.parse_internal()?;

        if self.lookahead_index < self.input.len() {
            Err("Syntax error: Invalid input".to_string())
        } else {
            Ok(())
        }
    }

    fn parse_internal(&mut self) -> Result<(), String> {
        match self.next_token() {
            Token::Zero => {
                self.parse_internal()?;
                self.match_token(Token::One)?;

                Ok(())
            }
            Token::One => {
                self.lookahead_index -= 1;
                Ok(())
            }
            Token::Epsilon => Ok(()),
            Token::Invalid => Err(format!(
                "Syntax error: Invalid token at position {}",
                self.lookahead_index
            )),
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
            '0' => {
                self.lookahead_index += 1;
                Token::Zero
            }
            '1' => {
                self.lookahead_index += 1;
                Token::One
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
    #[case("01")]
    #[case("0011")]
    #[case("00000000001111111111")]
    fn test_2_4_1_b_valid(#[case] input: &str) -> Result<(), String> {
        assert_eq!(ParserC::new(input).parse()?, ());
        Ok(())
    }

    #[rstest]
    #[case("0", "Syntax error: Expected One at position 1")]
    #[case("1", "Syntax error: Invalid input")]
    #[case("10", "Syntax error: Invalid input")]
    #[case("010", "Syntax error: Invalid input")]
    #[case("001", "Syntax error: Expected One at position 3")]
    #[case("00110", "Syntax error: Invalid input")]
    #[case("a", "Syntax error: Invalid token at position 0")]
    // #[case("()())", "Syntax error: Invalid input")]
    fn test_2_4_1_invalid(#[case] input: &str, #[case] error_message: &str) -> Result<(), String> {
        assert_eq!(
            ParserC::new(input).parse().unwrap_err(),
            error_message.to_string()
        );
        Ok(())
    }
}
