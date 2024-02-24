use ch02::ex2_4_1_b::ParserB;

use ch02::ex2_4_1_a::ParserA;

pub mod ch02;

fn main() {
    match ParserA::new("aa").parse() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }

    match ParserB::new("()()()").parse() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }
}
