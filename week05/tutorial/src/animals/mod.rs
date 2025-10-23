pub struct Cat {
    pub name: String,
}

pub trait Animal {
    fn sound(&self) -> String;
}

// Implement Animal trait for Cat
impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("Meow")
    }
}

