/// The goal is to take input and print it out as output
/// The input is string is of any length
use std::env;

fn main() {
    let input: Vec<String> = env::args().collect();

    for value in input.iter().skip(1) {
        print!("{} ", value);
    }
    println!()
}
