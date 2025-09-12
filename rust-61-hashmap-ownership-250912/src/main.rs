
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("Banana", "Yellow");

    let key = String::from("Apple");
    let value = String::from("Green");
    println!("The {key} is {value}");
    map.insert(&key, &value);
}
