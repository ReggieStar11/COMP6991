struct English;
struct Spanish;
struct French;

trait Greeting {
    fn greet(&self);
}

impl Greeting for English {
    fn greet(&self) {
        println!("Hello!");
    }
}

impl Greeting for Spanish {
    fn greet(&self) {
        println!("Hola!");
    }
}

impl Greeting for French {
    fn greet(&self) {
        println!("Bonjour!");
    }
}

struct Person {
    name: String,
    greetings: Vec<Box<dyn Greeting>>,
}

impl From<&str> for Box<dyn Greeting> {
    fn from(s: &str) -> Box<dyn Greeting> {
        match s {
            "English" => Box::new(English),
            "Spanish" => Box::new(Spanish),
            "French" => Box::new(French),
            _ => unreachable!(),
        }
    }
}

// DO NOT NEED TO CHANGE MAIN
fn main() {
    let person = Person {
        name: "John".to_string(),
        greetings: vec!["English".into(), "Spanish".into()],
    };

    speak_all_greetings(&person);

    let person = Person {
        name: "Jane".to_string(),
        greetings: vec!["French".into()],
    };

    speak_all_greetings(&person);
}

fn speak_all_greetings(person: &Person) {
    println!("{} says:", person.name);
    for g in &person.greetings {
        g.greet();
    }
}
