mod animals;
use animals::{Animal, Cat, Horse};

fn main() {

    let my_animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat {
            name: String::from("Whiskers"),
        }),
        Box::new(Horse {
            name: String::from("Thunder"),
        }),
    ];

    println!("Hello, world!");

    for animal in my_animals {
        println!("{:?} says {}", animal, animal.sound());
    }
}

fn read_blurbs(animals: Vec<Dog>) {
    
}