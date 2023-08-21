mod models;
use models::{Container,Get};
fn main() {
    let new_box = Container{
        weight: 20,
        id: "dfjldsnfl",
    };
    println!("{}", new_box);
}
