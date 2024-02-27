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

    match ParserC::new("01").parse() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }

    let tokens = Lexer::new(
        "// random  comment \n\
        hello = 12    * 5\t + 3\n\
        boolean_variable_=true | false //comment at the /* end\n\
        /* test multiline comment\n\
        commented_variable = 3 */\n
        > >= < <= == !=\n
        2. 3.14 .5",
    )
    .tokenize();
    print_tokens(tokens);
}

fn print_tokens(tokens: Vec<Token>) {
    for token in tokens.iter() {
        match token {
            Token::Word(tag, lexeme) => println!("{:?} - {}", tag, lexeme),
            Token::Number(integer, decimal) => println!("Number - {}.{}", integer, decimal),
            Token::LogicalOperator(tag, lexeme) => println!("{:?} - {}", tag, lexeme),
            Token::Unknown(lexeme) => println!("{}", lexeme),
            Token::Epsilon => println!("Epsilon"),
        }
    }
}
