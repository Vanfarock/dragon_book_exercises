use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Word {
    Identifier,
    True,
    False,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum LogicalOperator {
    Less,
    LessOrEqual,
    Equal,
    Different,
    Greater,
    GreaterOrEqual,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Token {
    Word(Word, String),
    Number(u32, u32),
    LogicalOperator(LogicalOperator, String),
    Unknown(String),
    Epsilon,
}

type Lexeme = String;

pub struct Lexer {
    input: Vec<char>,
    peek_index: usize,
    words: HashMap<Lexeme, Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut words = HashMap::new();
        for reserved_kw in Lexer::get_reserved_keywords().into_iter() {
            match &reserved_kw {
                Token::Word(_, lexeme) => {
                    words.insert(lexeme.clone(), reserved_kw);
                }
                _ => (),
            }
        }

        Lexer {
            input: input.chars().collect(),
            peek_index: 0,
            words,
        }
    }

    fn get_reserved_keywords() -> Vec<Token> {
        vec![
            Token::Word(Word::True, "true".to_string()),
            Token::Word(Word::False, "false".to_string()),
        ]
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut next_token = self.scan();
        while next_token != Token::Epsilon {
            tokens.push(next_token);
            next_token = self.scan();
        }
        tokens
    }

    fn scan(&mut self) -> Token {
        while self.peek_index < self.input.len() {
            let peek = self.input[self.peek_index];

            if self.is_whitespace() {
                self.move_peek();
                continue;
            }
            if self.is_single_line_comment() {
                self.handle_single_line_comment();
                continue;
            }
            if self.is_multi_line_comment() {
                self.handle_multi_line_comment();
                continue;
            }

            if self.is_number() {
                return self.handle_number();
            }
            if peek.is_alphabetic() {
                return self.handle_word();
            }

            if let Some(logical_operator) = self.is_logical_operator() {
                return logical_operator;
            }

            self.move_peek();
            return Token::Unknown(peek.to_string());
        }

        Token::Epsilon
    }

    fn is_whitespace(&self) -> bool {
        let peek = self.input[self.peek_index];
        peek == ' ' || peek == '\t' || peek == '\n'
    }

    fn is_single_line_comment(&mut self) -> bool {
        self.match_sequence("//")
    }

    fn handle_single_line_comment(&mut self) {
        while self.move_peek() {
            if self.input[self.peek_index] == '\n' {
                return;
            }
        }
    }
    fn is_multi_line_comment(&mut self) -> bool {
        self.match_sequence("/*")
    }
    fn handle_multi_line_comment(&mut self) {
        while self.move_peek() {
            if self.match_sequence("*/") {
                return;
            }
        }
    }

    fn is_number(&mut self) -> bool {
        let mut peek = self.input[self.peek_index];
        if peek.is_numeric() {
            return true;
        }

        let mut is_numeric = false;
        if peek == '.' {
            if self.move_peek() {
                peek = self.input[self.peek_index];

                if peek.is_numeric() {
                    is_numeric = true;
                }
            }
            self.move_peek_reverse();
        }
        is_numeric
    }

    fn handle_number(&mut self) -> Token {
        let mut peek = self.input[self.peek_index];
        let mut is_float = peek == '.';
        let mut integer = if is_float {
            0
        } else {
            peek.to_digit(10).unwrap()
        };
        let mut decimal = 0;

        if self.move_peek() {
            peek = self.input[self.peek_index];

            while peek.is_numeric() || peek == '.' {
                if peek == '.' {
                    if is_float {
                        break;
                    }
                    is_float = true;
                } else if is_float {
                    decimal = decimal * 10 + peek.to_digit(10).unwrap();
                } else {
                    integer = integer * 10 + peek.to_digit(10).unwrap();
                }

                if !self.move_peek() {
                    break;
                }
                peek = self.input[self.peek_index];
            }
        }
        Token::Number(integer, decimal)
    }

    fn handle_word(&mut self) -> Token {
        let mut peek = self.input[self.peek_index];
        let mut word = peek.to_string();
        if self.move_peek() {
            peek = self.input[self.peek_index];

            while peek.is_alphanumeric() || peek == '_' {
                word.push(peek);
                if !self.move_peek() {
                    break;
                }
                peek = self.input[self.peek_index];
            }
        }
        if let Some(word_token) = self.words.get(&word) {
            return word_token.clone();
        }
        let new_identifier = Token::Word(Word::Identifier, word.to_string());
        self.words.insert(word, new_identifier.clone());
        new_identifier
    }

    fn is_logical_operator(&mut self) -> Option<Token> {
        if self.match_sequence("<=") {
            Some(Token::LogicalOperator(
                LogicalOperator::LessOrEqual,
                "<=".to_string(),
            ))
        } else if self.match_sequence("==") {
            Some(Token::LogicalOperator(
                LogicalOperator::Equal,
                "==".to_string(),
            ))
        } else if self.match_sequence("!=") {
            Some(Token::LogicalOperator(
                LogicalOperator::Different,
                "!=".to_string(),
            ))
        } else if self.match_sequence(">=") {
            Some(Token::LogicalOperator(
                LogicalOperator::GreaterOrEqual,
                ">=".to_string(),
            ))
        } else if self.match_sequence("<") {
            Some(Token::LogicalOperator(
                LogicalOperator::Less,
                "<".to_string(),
            ))
        } else if self.match_sequence(">") {
            Some(Token::LogicalOperator(
                LogicalOperator::Greater,
                ">".to_string(),
            ))
        } else {
            None
        }
    }

    fn match_sequence(&mut self, expected_sequence: &str) -> bool {
        let initial_peek_index = self.peek_index;
        let mut peek = self.input[self.peek_index];
        let chars: Vec<char> = expected_sequence.chars().collect();
        let mut moved = false;

        for sequence_char in chars.into_iter() {
            if peek == sequence_char {
                if self.move_peek() {
                    peek = self.input[self.peek_index];
                    moved = true;
                }
            } else {
                if moved {
                    self.move_peek_to(initial_peek_index);
                }
                return false;
            }
        }
        true
    }

    fn move_peek(&mut self) -> bool {
        self.peek_index += 1;
        self.peek_index < self.input.len()
    }
    fn move_peek_reverse(&mut self) {
        self.peek_index -= 1;
    }
    fn move_peek_to(&mut self, new_peek: usize) {
        self.peek_index = new_peek;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_lexer() {
        let mut lexer = Lexer::new(
            "// random  comment \n\
             hello = 12    * 5\t + 3\n\
             boolean_variable_=true | false //comment at the /* end\n\
             /* test multiline comment\n\
             commented_variable = 3 */\n\
             > >= < <= == !=\n\
             >>=<<===!=\n\
             2. 3.14 .5.",
        );
        let tokens: Vec<Token> = lexer.tokenize();

        let expected_values = vec![
            Token::Word(Word::Identifier, "hello".to_string()),
            Token::Unknown("=".to_string()),
            Token::Number(12, 0),
            Token::Unknown("*".to_string()),
            Token::Number(5, 0),
            Token::Unknown("+".to_string()),
            Token::Number(3, 0),
            Token::Word(Word::Identifier, "boolean_variable_".to_string()),
            Token::Unknown("=".to_string()),
            Token::Word(Word::True, "true".to_string()),
            Token::Unknown("|".to_string()),
            Token::Word(Word::False, "false".to_string()),
            Token::LogicalOperator(LogicalOperator::Greater, ">".to_string()),
            Token::LogicalOperator(LogicalOperator::GreaterOrEqual, ">=".to_string()),
            Token::LogicalOperator(LogicalOperator::Less, "<".to_string()),
            Token::LogicalOperator(LogicalOperator::LessOrEqual, "<=".to_string()),
            Token::LogicalOperator(LogicalOperator::Equal, "==".to_string()),
            Token::LogicalOperator(LogicalOperator::Different, "!=".to_string()),
            Token::LogicalOperator(LogicalOperator::Greater, ">".to_string()),
            Token::LogicalOperator(LogicalOperator::GreaterOrEqual, ">=".to_string()),
            Token::LogicalOperator(LogicalOperator::Less, "<".to_string()),
            Token::LogicalOperator(LogicalOperator::LessOrEqual, "<=".to_string()),
            Token::LogicalOperator(LogicalOperator::Equal, "==".to_string()),
            Token::LogicalOperator(LogicalOperator::Different, "!=".to_string()),
            Token::Number(2, 0),
            Token::Number(3, 14),
            Token::Number(0, 5),
            Token::Unknown(".".to_string()),
        ];

        for (i, token) in tokens.into_iter().enumerate() {
            assert_eq!(token, expected_values[i]);
        }
    }
}
