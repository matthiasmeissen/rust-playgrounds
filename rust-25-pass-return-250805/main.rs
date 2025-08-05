
fn main() {
    let mut my_string = give_ownership();
    println!("String in main is: {my_string}");
}

fn give_ownership() -> String {
    let generated_string = String::from("Hello");
    generated_string
}
