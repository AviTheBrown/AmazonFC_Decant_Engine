pub mod models;

use models::{Container,MetaData, Tote};
fn main() {
    let new_container = Tote::new();
    println!("{}", new_container.unwrap());
}
