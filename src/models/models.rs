use std::fmt;

#[derive(Clone)]
pub struct Container<'a> {
    pub weight: u32,
    pub id: &'a str,
}

#[derive(Clone)]
pub struct Tote<'a> {
    pub number_of_items: u32,
    pub weight: u32,
    pub id: &'a str,
}

pub trait Get {
    fn get(&self) -> &Self;
}

impl<'a> Get for Container<'a> {
    fn get(&self) -> &Self {
        self
    }
}

impl<'a> Get for Tote<'a> {
    fn get(&self) -> &Self {
        self
    }
}

impl<'a> fmt::Display for Container<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Container test {{weigth: {}, ID: {} }}", self.weight, self.id)
    }
}
impl<'a> fmt::Display for Tote<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tote: {{Number of items currently: {}, Weight: {}, ID: {} }}",
            self.weight, self.number_of_items, self.id
        )
    }
}
