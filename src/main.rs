mod models;
use models::Box;
use models::PullData;

fn main() {
    let new_box = Box {
        weight: 20,
        id: "dfjldsnfl".to_string(),
    };
    new_box.pull_data();
}
