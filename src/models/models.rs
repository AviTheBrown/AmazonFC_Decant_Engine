use rand::Rng;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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
// -> Used only on Tote instances
pub enum ToteError {
    // Weight should not exceed more than 25lbs
    TooMuchWeight(/* tote id */ String, /* tote weight */ i32),
    // If there is a any problem with a tote, i.e invalid or being used (duplicate id)
    DirtyTote(String),
}
enum ModelType {
    Container(Container),
    Tote(Tote),
}
// type ModelResult = Result<ToteOrContainer, NonImplOnTypeErr>;

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub weight: u32,
    id: Rc<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tote {
    pub number_of_items: u32,
    pub weight: u32,
    id: Rc<String>,
}

pub trait ModelData {
    fn get(&self) -> &Self;
    fn id_gen() -> String;
}

impl ModelData for Container {
    fn get(&self) -> &Self {
        self
    }
    fn id_gen() -> String {
        let length = 16;
        let char_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456789"
            .chars()
            .collect::<Vec<char>>();
        let mut rng = rand::thread_rng();
        let id: String = (0..length)
            .map(|_| char_set[rng.gen_range(0..char_set.len())])
            .collect();
        id
    }
}
impl Container {
    pub fn new() -> Self {
        Container {
            weight: 0,
            id: Rc::new(Container::id_gen()),
        }
    }
}

impl ModelData for Tote {
    fn get(&self) -> &Self {
        self
    }
    fn id_gen() -> String {
        let length = 16;
        let char_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456789"
            .chars()
            .collect::<Vec<char>>();
        let mut rng = rand::thread_rng();
        let id: String = (0..length)
            .map(|_| char_set[rng.gen_range(0..char_set.len())])
            .collect();
        id
    }
}

impl Tote {
    //fn update_tote(self) -> Result<Self, ToteError> {}
    pub fn new() -> Self {
        Tote {
            number_of_items: 0,
            weight: 0,
            id: Rc::new(Tote::id_gen()),
        }
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Container test {{weigth: {}, ID: {} }}",
            self.weight, self.id
        )
    }
}
impl fmt::Display for Tote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tote: {{Number of items currently: {}, Weight: {}, ID: {} }}",
            self.weight, self.number_of_items, self.id
        )
    }
}
