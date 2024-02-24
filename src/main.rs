use ch02::ex2_4_1_b::Parser;

use ch02::ex2_4_1_a::a;

pub mod ch02;

fn main() {
    match a("a") {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }

    match Parser::new("()()()").parse() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }
}
