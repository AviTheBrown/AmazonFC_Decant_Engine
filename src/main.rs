pub mod models;

use models::{Container, MetaData, Tote};
fn main() {
    let mut new_container = Tote::new().unwrap();
    // new_container.weight = 30;
    println!("{}", new_container);

    new_container.id = "fdkljfnkdsf9fns9";
    println!("{}", new_container)
}
