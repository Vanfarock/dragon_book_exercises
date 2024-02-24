pub fn a(input: &str) -> Result<(), String> {
    // Recursive-descent parser for the following grammar:
    // S := +SS | -SS | a

    let lookahead_index = 0;

    match s_a(input, lookahead_index) {
        Ok(last_lookahead_index) => {
            if input.len() > last_lookahead_index {
                Err("Syntax Error: Expression finished earlier than expected".to_string())
            } else {
                Ok(())
            }
        }
        Err(err) => Err(format!("Syntax Error: {}", err)),
    }
}

fn s_a(input: &str, mut lookahead_index: usize) -> Result<usize, String> {
    if let Some(token) = input.chars().nth(lookahead_index) {
        match token {
            '+' | '-' => {
                lookahead_index += 1;
                lookahead_index = s_a(input, lookahead_index)?;
                lookahead_index = s_a(input, lookahead_index)?;
                Ok(lookahead_index)
            }
            'a' => {
                lookahead_index += 1;
                Ok(lookahead_index)
            }
            _ => Err(format!("Invalid token at position {}", lookahead_index)),
        }
    } else {
        Err(format!("Expected more tokens"))
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
        assert_eq!(a(input)?, ());

        Ok(())
    }

    #[rstest]
    #[case("++aa", "Syntax Error: Expected more tokens")]
    #[case("+a", "Syntax Error: Expected more tokens")]
    #[case("+ab", "Syntax Error: Invalid token at position 2")]
    #[case("+ba", "Syntax Error: Invalid token at position 1")]
    #[case("--aa", "Syntax Error: Expected more tokens")]
    #[case("-a", "Syntax Error: Expected more tokens")]
    #[case("-ab", "Syntax Error: Invalid token at position 2")]
    #[case("-ba", "Syntax Error: Invalid token at position 1")]
    #[case("aa", "Syntax Error: Expression finished earlier than expected")]
    #[case("b", "Syntax Error: Invalid token at position 0")]
    fn test_2_4_1_a_invalid(
        #[case] input: &str,
        #[case] error_message: &str,
    ) -> Result<(), String> {
        assert_eq!(a(input).unwrap_err(), error_message.to_string());

        Ok(())
    }
}
