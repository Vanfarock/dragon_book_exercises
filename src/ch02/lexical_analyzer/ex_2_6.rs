use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    True,
    False,
    Number,
    Identifier,
    Unknown,
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

pub struct Lexer {
    input: Vec<char>,
    peek_index: usize,
    words: HashMap<Lexeme, Word>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut words = HashMap::new();
        for reserved_kw in Lexer::get_reserved_key_words().into_iter() {
            words.insert(reserved_kw.get_lexeme().unwrap(), reserved_kw);
        }

        Lexer {
            input: input.chars().collect(),
            peek_index: 0,
            words,
        }
    }

    fn get_reserved_key_words() -> Vec<Word> {
        vec![
            Word::new(Tag::True, "true".to_string()),
            Word::new(Tag::False, "false".to_string()),
        ]
    }

    pub fn scan(&mut self) -> Box<dyn Token> {
        while self.peek_index < self.input.len() {
            let mut peek = self.input[self.peek_index];

            if self.is_whitespace(peek) {
                self.peek_index += 1;
                continue;
            }

            if peek.is_numeric() {
                let mut value = peek.to_digit(10).unwrap();
                self.peek_index += 1;
                peek = self.input[self.peek_index];

                while peek.is_numeric() {
                    value = value * 10 + peek.to_digit(10).unwrap();
                    self.peek_index += 1;
                    peek = self.input[self.peek_index];
                }
                return Box::new(Num::new(value));
            }

            if peek.is_alphabetic() {
                let mut word = peek.to_string();
                self.peek_index += 1;
                peek = self.input[self.peek_index];

                while peek.is_alphanumeric() {
                    word.push(peek);
                    self.peek_index += 1;
                    peek = self.input[self.peek_index];
                }
                if let Some(word_token) = self.words.get(&word) {
                    return Box::new(word_token.clone());
                }
            }

            return Box::new(Unknown::new(peek));
        }

        Box::new(Unknown::new(' '))
    }

    fn is_whitespace(&self, peek: char) -> bool {
        peek == ' ' || peek == '\t' || peek == '\n'
    }
}
