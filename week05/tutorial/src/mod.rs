mod cat;

pub struct Cat {
    pub name: String,
}

impl Animal for Horse {
    fn sound(&self) -> &'static str {
        "neigh"
    }
}