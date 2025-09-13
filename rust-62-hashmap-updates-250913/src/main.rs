use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Bus"), 80);
    map.insert(String::from("Car"), 10);
    print_map(&map);

    // Insert a new key and value into the map
    // When key alreads exists will overwrite the existing
    map.insert(String::from("Car"), 20);
    print_map(&map);

    // The entry method checks if a key already exists and returns a type Entry
    let bike = map.entry(String::from("Bike"));
    // On the Entry type we can call the .or_insert() method
    // Which leaves the values as it is when exist, or add a new key with that value
    bike.or_insert(200);
    map.entry(String::from("Bus")).or_insert(10);
    print_map(&map);

    let mut map1 = HashMap::new();
    let text = "Hello people on this people planet";

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map1);
}

fn print_map(map: &HashMap<String, i32>) {
    println!("--------");
    for (key, value) in map {
        println!("{key} has {value} points");
    }
}