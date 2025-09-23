use std::io::{self, Write};

fn main() {
    print!("What is your name? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("No name entered :(, goodbye.");

    let name = input.trim();

    if name.is_empty() {
        println!("No name entered :(, goodbye.");
    } else {
        println!("Hello, {}, nice to meet you!", name);
    }
}
