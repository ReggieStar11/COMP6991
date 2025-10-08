use std::io;

fn main() {
    println!("What is your name?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("No name entered :(, goodbye.");

    // Remove the newline character at the end
    let name = input.trim();

    println!("Hello, {}, nice to meet you!", name);
}
