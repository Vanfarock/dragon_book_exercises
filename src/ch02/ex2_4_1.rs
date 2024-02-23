pub fn a(input: &str) -> Result<(), String> {
    let lookahead_index = 0;

    s_a(input, lookahead_index).map(|_| ())
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
            _ => Err("Syntax Error".to_string()),
        }
    } else {
        Err("Syntax Error".to_string())
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
    #[case("-aaa")]
    fn test_2_4_1_valid(#[case] input: &str) -> Result<(), String> {
        assert_eq!(a(input)?, ());

        Ok(())
    }

    #[rstest]
    #[case("++aa")]
    #[case("+a")]
    #[case("+ab")]
    #[case("+ba")]
    #[case("--aa")]
    #[case("-a")]
    #[case("-ab")]
    #[case("-ba")]
    #[case("b")]
    fn test_2_4_1_invalid(#[case] input: &str) -> Result<(), String> {
        assert_eq!(a(input).unwrap_err(), "Syntax Error".to_string());

        Ok(())
    }
}
