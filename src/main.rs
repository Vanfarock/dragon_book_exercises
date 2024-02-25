use ch02::lexical_analyzer::ex_2_6::{Lexer, Token};
use ch02::recursive_descent_parser::ex2_4_1_a::ParserA;
use ch02::recursive_descent_parser::ex2_4_1_b::ParserB;
use ch02::recursive_descent_parser::ex2_4_1_c::ParserC;

pub mod ch02;

fn main() {
    match ParserA::new("+aa").parse() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }

    match ParserB::new("()()()").parse() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }

    match ParserC::new("0").parse() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }

    let tokens = Lexer::new("test = 12").tokenize();
    print_tokens(tokens);
}

fn print_tokens(tokens: Vec<Box<dyn Token>>) {
    for token in tokens.iter() {
        if let Some(lexeme) = token.get_lexeme() {
            println!("{:#?} - {}", token.get_tag(), lexeme);
        } else {
            println!("{:#?}", token.get_tag());
        }
    }
}
