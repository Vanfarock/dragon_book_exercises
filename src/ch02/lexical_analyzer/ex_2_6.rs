use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    True,
    False,
    Number,
    Identifier,
    Unknown,
    Epsilon,
}

type Lexeme = String;

pub trait Token {
    fn get_tag(&self) -> Tag;
    fn get_lexeme(&self) -> Option<String>;
}

#[derive(Clone)]
pub struct Word {
    tag: Tag,
    lexeme: String,
}

impl Word {
    fn new(tag: Tag, lexeme: String) -> Self {
        Word { tag, lexeme }
    }
}

impl Token for Word {
    fn get_tag(&self) -> Tag {
        self.tag
    }

    fn get_lexeme(&self) -> Option<String> {
        Some(self.lexeme.clone())
    }
}

pub struct Num {
    tag: Tag,
    value: u32,
}

impl Num {
    fn new(value: u32) -> Self {
        Num {
            tag: Tag::Number,
            value,
        }
    }
}

impl Token for Num {
    fn get_tag(&self) -> Tag {
        self.tag
    }

    fn get_lexeme(&self) -> Option<String> {
        Some(self.value.to_string())
    }
}

pub struct Unknown {
    lexeme: char,
}

impl Unknown {
    fn new(lexeme: char) -> Self {
        Unknown { lexeme }
    }
}

impl Token for Unknown {
    fn get_tag(&self) -> Tag {
        Tag::Unknown
    }

    fn get_lexeme(&self) -> Option<String> {
        Some(self.lexeme.to_string())
    }
}
pub struct Epsilon {}

impl Epsilon {
    fn new() -> Self {
        Epsilon {}
    }
}

impl Token for Epsilon {
    fn get_tag(&self) -> Tag {
        Tag::Epsilon
    }

    fn get_lexeme(&self) -> Option<String> {
        None
    }
}

pub struct Lexer {
    input: Vec<char>,
    peek_index: usize,
    words: HashMap<Lexeme, Word>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut words = HashMap::new();
        for reserved_kw in Lexer::get_reserved_keywords().into_iter() {
            words.insert(reserved_kw.get_lexeme().unwrap(), reserved_kw);
        }

        Lexer {
            input: input.chars().collect(),
            peek_index: 0,
            words,
        }
    }

    fn get_reserved_keywords() -> Vec<Word> {
        vec![
            Word::new(Tag::True, "true".to_string()),
            Word::new(Tag::False, "false".to_string()),
        ]
    }

    pub fn tokenize(&mut self) -> Vec<Box<dyn Token>> {
        let mut tokens = Vec::new();
        let mut next_token = self.scan();
        while next_token.get_tag() != Tag::Epsilon {
            tokens.push(next_token);
            next_token = self.scan();
        }
        tokens
    }

    fn scan(&mut self) -> Box<dyn Token> {
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

            if peek.is_numeric() {
                return self.handle_number();
            }
            if peek.is_alphabetic() {
                return self.handle_word();
            }

            self.move_peek();
            return Box::new(Unknown::new(peek));
        }

        Box::new(Epsilon::new())
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

    fn handle_number(&mut self) -> Box<dyn Token> {
        let mut peek = self.input[self.peek_index];
        let mut value = peek.to_digit(10).unwrap();
        if self.move_peek() {
            peek = self.input[self.peek_index];

            while peek.is_numeric() {
                value = value * 10 + peek.to_digit(10).unwrap();
                if !self.move_peek() {
                    break;
                }
                peek = self.input[self.peek_index];
            }
        }
        Box::new(Num::new(value))
    }

    fn handle_word(&mut self) -> Box<dyn Token> {
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
            return Box::new(word_token.clone());
        }
        let new_identifier = Word::new(Tag::Identifier, word.to_string());
        self.words.insert(word, new_identifier.clone());
        Box::new(new_identifier)
    }

    fn match_sequence(&mut self, expected_sequence: &str) -> bool {
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
                    self.move_peek_reverse();
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
             commented_variable = 3 */",
        );
        let tokens: Vec<Box<dyn Token>> = lexer.tokenize();

        let expected_values = vec![
            (Tag::Identifier, Some("hello".to_string())),
            (Tag::Unknown, Some("=".to_string())),
            (Tag::Number, Some("12".to_string())),
            (Tag::Unknown, Some("*".to_string())),
            (Tag::Number, Some("5".to_string())),
            (Tag::Unknown, Some("+".to_string())),
            (Tag::Number, Some("3".to_string())),
            (Tag::Identifier, Some("boolean_variable_".to_string())),
            (Tag::Unknown, Some("=".to_string())),
            (Tag::True, Some("true".to_string())),
            (Tag::Unknown, Some("|".to_string())),
            (Tag::False, Some("false".to_string())),
        ];

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.get_tag(), expected_values[i].0);
            assert_eq!(token.get_lexeme(), expected_values[i].1);
        }
    }
}
