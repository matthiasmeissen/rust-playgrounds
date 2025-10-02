
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

    print_summary(&article1);
}

// The print_summary function can take any type as parameter that implements the Trait Summary
fn print_summary (item: &impl Summary) {
    println!("{}", item.summarize());
}
