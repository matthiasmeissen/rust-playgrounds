
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("Team 1", 20);
    map.insert("Team 2", 10);

    for (key, value) in &map {
        println!("{key} as {value} points");
    }
}
