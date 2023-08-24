pub mod models;
use std::rc::Rc;

use models::{Container, ModelData, Tote};
fn main() {
    let mut new_container = Tote::new();
    // new_container.weight = 30;
    println!("{}", new_container);
    new_container.weight = 30;
    println!("{}", new_container)
}
