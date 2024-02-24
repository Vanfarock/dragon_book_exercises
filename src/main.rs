use ch02::ex2_4_1_b::ParserB;

use ch02::ex2_4_1_a::ParserA;
use ch02::ex2_4_1_c::ParserC;

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
}
