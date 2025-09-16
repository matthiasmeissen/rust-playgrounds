
use std::collections::HashMap;

pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn add_data(&mut self, name: String, department: String) {
        self.data.insert(name, department);
        println!("Added elements");
    }

    pub fn print_data(&self) {
        println!("--------");
        for entry in &self.data {
            println!("{} from {}", entry.0, entry.1)
        }
        println!("--------");
    }
}
