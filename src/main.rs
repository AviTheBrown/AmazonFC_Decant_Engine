pub mod models;

use models::{Container, MetaData, Tote};
fn main() {
    let mut new_container = Tote::new();
    // new_container.weight = 30;
    println!("{}", new_container);

    println!("{}", new_container)
}
