use std::io::{self};

fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    println!("The command-line argument is: {pattern}");
}
