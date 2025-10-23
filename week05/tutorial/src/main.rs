mod animals;

use crate::animals::{Animal, Cat};

fn main() {
    let my_animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat {
            name: String::from("Whiskers"),
        }),
        // Box::new(Horse {
        //     name: String::from("Thunder"),
        // }),
    ];

    println!("Hello, world!");

    read_blurbs(my_animals);
    read_blurbs_1(my_animals);
}

fn read_blurbs(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("Animal says {}", animal.sound());
    }
}

fn read_blurbs_1(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        println!("Animal says {}", animal.sound());
    }
}

fn read_blurbs_2<T>(animals: T)
where
    T: IntoIterator<Item = Box<dyn Animal>>,
{
    for animal in animals {
        println!("Animal says {}", animal.sound());
    }
}

fn read_blurbs_3<T, A>(animals: T)
where
    T: IntoIterator<Item = A>,
    A: std::ops::Deref,
    A::Target: Animal,
{
    for animal in animals {
        println!("Animal says {}", animal.sound());
    }
}

fn read_blurbs_4<'a, T, U>(animals: T)
where
    T: IntoIterator<Item = A>,
    A: std::ops::Deref,
    A::Target: Animal,
{
    for animal in animals {
        println!("Animal says {}", animal.sound());
    }
}