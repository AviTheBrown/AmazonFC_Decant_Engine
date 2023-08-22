use std::fmt;

#[derive(Debug)]
// -> Only used for get and new methods for model type
// NoneCreation should only be used when there is a problem with creating
// an instance of the type
// NonCorrectType should only be used when a new method is used on an already
// existing instance. -> new_tote::new() -> new_tote.get() :is ok | new_tote::new() -> new_tote::new() :is not ok!!
pub enum NonImplOnTypeErr {
    NonCreation(String),
    NonCorrectType(String),
}
pub enum ToteError {
    TooMuchWeight(Stirng, i32),
    DirtyTote(String)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Container{
    pub weight: u32,
    pub id: Rc<RefCell<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tote {
    pub number_of_items: u32,
    pub weight: u32,
    pub id: Rc<RefCell<String>>,
}

pub trait MetaData {
    fn get(&self) -> &Self;
    fn new() -> Result<Self, NonImplOnTypeErr>
    where
        Self: Sized;
}

impl<'a> MetaData for Container<'a> {
    fn get(&self) -> &Self {
        self
    }
    fn new() -> Result<Self, NonImplOnTypeErr> {
        self
}

impl<'a> MetaData for Tote<'a> {
    fn get(&self) -> &Self {
        self
    }
    fn new() -> Result<Self, NonImplOnTypeErr> {
        let id_generator = "";
        Container {
            number_of_items: 0,
            weight: 0,
            id: ""
        }
        Err
    }
}

impl<'a> Tote<'a> {
    fn update_tote(&self) -> Result<&Self, ToteError> {
        
    }
}

impl<'a> fmt::Display for Container<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Container test {{weigth: {}, ID: {} }}",
            self.weight, self.id
        )
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
