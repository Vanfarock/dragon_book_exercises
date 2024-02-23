use crate::ch02::ex2_4_1::a;

pub mod ch02;

fn main() {
    match a("a") {
        Ok(_) => println!("Success!"),
        Err(err) => println!("{}", err),
    }
}
