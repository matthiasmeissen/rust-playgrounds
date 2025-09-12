use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Banana"), String::from("Yellow"));
    
    let key = String::from("Apple");
    let value = String::from("Green");
    map.insert(key, value);

    // You can not access the variables key and value
    // Since the map has taken ownership over them
    // This is only true for types that do not implement the copy trait
    //println!("The {key} is {value}");

    // But you can still iterate over the map
    for (k, v) in &map {
        println!("The {k} is {v}");
    }

    let mut map_int = HashMap::new();
    map_int.insert(4, 20);

    // For types that have the copy trait 
    // you can still access them after you create a hasmap with them
    // since they get copied and then added to it
    let key2 = 8;
    let value2 = 20;
    map_int.insert(key2, value2);
    println!("The {key2} is {value2}");
}