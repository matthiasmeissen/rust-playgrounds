use std::fmt::Debug;

// Specify trait with name Summary to include summarize method
trait Summary {
    // The summarize method is called on the type it is implemented on and returns a String
    fn summarize(&self) -> String;
}

// Define Article struct with two members
struct Article {
    title: String,
    subtitle: String,
}

// Implement Summary Trait for Article by defining summarize method
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Summarize Article: {}, {}", self.title, self.subtitle)
    }
}

// Define Social struct with two members
#[derive(Debug)]
struct Social {
    title: String,
    caption: String,
}

// Implement Summary Trait for Social by defining summarize method
impl Summary for Social {
    fn summarize(&self) -> String {
        format!("Summarize Post: {}, {}", self.title, self.caption)
    }
}

fn main() {
    let article1 = Article {
        title: String::from("Article Title"),
        subtitle: String::from("Article Subtitle")
    };

    let social1 = create_social();

    print_summary(&article1);

    // This wont work, since the Article struct does not implement the Trait Debug
    //debug_element(&article1);

    debug_element(&social1);
}

// The print_summary function can take any type as parameter that implements the Trait Summary
fn print_summary (item: &impl Summary) {
    println!("{}", item.summarize());
}

// You can also specify the return type of a function
// Note that you need to explicitly state that it also includes the Trait Debug
// Even when Social struct has it
fn create_social() -> impl Summary + Debug {
    Social {
        title: String::from("Social Title"),
        caption: String::from("Social Caption")
    }
}

fn debug_element (item: &impl Debug) {
    println!("{:?}", item);
}