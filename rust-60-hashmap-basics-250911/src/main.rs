// To use hash maps you need to import them from the standard libraray
use std::collections::HashMap;

fn main() {
    // Create a new empty hash map with HashMap::new()
    let mut scores = HashMap::new();
    // Add new entry to the has map with .insert() method
    // A hash map entry consist of a key value pair
    // Where either of them can be of any type, but mus stay consistent within a hash map
    scores.insert(String::from("Team 1"), 20);
    scores.insert(String::from("Team 2"), 40);

    // Create String of team name we want to search the score for
    let team_name = String::from("Team 1");
    // The .get() method on a hash map returns an Option<&T> with a reference to the value
    // Since we might not have the entry within the has map
    let score = scores.get(&team_name);
    // We can use the match control flow to specify what we want to do with the value
    match score {
        Some(i) => println!("The scores of {team_name} is {i}."),
        None => println!("{team_name} does not exist in this has map."),
    }
    // We can also use the .copied() method on the .get() to copy and get owenership over the value
    // The .unwrap_or() method means that we return the value when present, or use the specified defaut when not
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of {team_name} is {score}.");

    // We can iterate over a key value pair using a for loop, similar to vector types
    for (key, value) in &scores {
        println!("{key} has a score of {value}");
    }
}