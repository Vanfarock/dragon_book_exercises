#[derive(Debug, PartialEq)]
enum Token {
    A,
    Plus,
    Minus,
    Epsilon,
    Invalid,
}

pub struct ParserA {
    input: Vec<char>,
    lookahead_index: usize,
}

impl ParserA {
    // Recursive-descent parser for the following grammar:
    // S := +SS | -SS | a

    pub fn new(input: &str) -> Self {
        ParserA {
            input: input.chars().collect(),
            lookahead_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let is_correct = self.parse_internal()?;
        if is_correct {
            if self.lookahead_index < self.input.len() {
                Err("Syntax error: Invalid input".to_string())
            } else {
                Ok(())
            }
        } else {
            Err("Syntax error: Invalid input".to_string())
        }
    }

    fn parse_internal(&mut self) -> Result<bool, String> {
        match self.next_token() {
            Token::A => Ok(true),
            Token::Plus | Token::Minus => {
                if !self.parse_internal()? {
                    return Ok(false);
                }
                if !self.parse_internal()? {
                    return Ok(false);
                }
                Ok(true)
            }
            Token::Epsilon => Ok(false),
            Token::Invalid => Err(format!(
                "Syntax error: Invalid token at position {}",
                self.lookahead_index
            )),
        }
    }

    fn next_token(&mut self) -> Token {
        if self.lookahead_index >= self.input.len() {
            return Token::Epsilon;
        }

        match self.input[self.lookahead_index] {
            'a' => {
                self.lookahead_index += 1;
                Token::A
            }
            '+' => {
                self.lookahead_index += 1;
                Token::Plus
            }
            '-' => {
                self.lookahead_index += 1;
                Token::Minus
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
    #[case("a")]
    #[case("+aa")]
    #[case("++++aaaaa")]
    #[case("+-aaa")]
    #[case("+++--+-+-+-+-+++-a-aaaaaaaaaaaaaaaaaa")]
    #[case("-aa")]
    fn test_2_4_1_a_valid(#[case] input: &str) -> Result<(), String> {
        assert_eq!(ParserA::new(input).parse()?, ());

        Ok(())
    }

    #[rstest]
    #[case("++aa", "Syntax error: Invalid input")]
    #[case("+a", "Syntax error: Invalid input")]
    #[case("+ab", "Syntax error: Invalid token at position 2")]
    #[case("+ba", "Syntax error: Invalid token at position 1")]
    #[case("--aa", "Syntax error: Invalid input")]
    #[case("-a", "Syntax error: Invalid input")]
    #[case("-ab", "Syntax error: Invalid token at position 2")]
    #[case("-ba", "Syntax error: Invalid token at position 1")]
    #[case("aa", "Syntax error: Invalid input")]
    #[case("b", "Syntax error: Invalid token at position 0")]
    fn test_2_4_1_a_invalid(
        #[case] input: &str,
        #[case] error_message: &str,
    ) -> Result<(), String> {
        assert_eq!(
            ParserA::new(input).parse().unwrap_err(),
            error_message.to_string()
        );

        Ok(())
    }
}
