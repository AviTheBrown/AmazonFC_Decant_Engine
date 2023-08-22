use std::fmt;

#[derive(Debug)]
pub enum NonImplOnTypeErr {
    NonCreation(String),
    NonCorrectType(String),
}

#[derive(Debug,Clone)]
pub struct Container<'a> {
    pub weight: u32,
    pub id: &'a str,
}

#[derive(Debug, Clone)]
pub struct Tote<'a> {
    pub number_of_items: u32,
    pub weight: u32,
    pub id: &'a str,
}

pub trait MetaData{
    fn get(&self) -> &Self;
    fn new() -> Result<Self, NonImplOnTypeErr> 
        where
         Self: Sized;
}

impl<'a> MetaData for Container<'a> {
    fn get(&self) -> &Self {
        self
    }
    fn new() -> Result<Self,NonImplOnTypeErr> {
       Err(NonImplOnTypeErr::NonCorrectType("Default implementation of new() is not provided for this Container".to_string()))
    }
}

impl<'a> MetaData for Tote<'a> {
    fn get(&self) -> &Self {
        self
    }
    fn new() -> Result<Self, NonImplOnTypeErr> {
        Ok(Tote {
            number_of_items: 0, weight: 0, id: ""
        })
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
