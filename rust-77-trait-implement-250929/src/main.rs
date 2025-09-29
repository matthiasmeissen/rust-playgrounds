trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    subline: String,
    body: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, {}", self.headline, self.subline)
    }
}

impl Default for Article {
    fn default() -> Self {
        Self { 
            headline: String::from("Headline Placeholder"), 
            subline: String::from("Caption Placeholder"), 
            body: String::from("Here is some body of the article.") 
        }
    }
}

struct Social {
    author: String,
    caption: String,
    shared: bool,
}

impl Summary for Social {
    fn summarize(&self) -> String {
        format!("{}, {}", self.author, self.caption)
    }
}


fn main() {
    let post = Social{
        author: String::from("Some Author"),
        caption: String::from("This is a post"),
        shared: false,
    };

    print_summary(post);

    let new_article = Article::default();
    print_summary(new_article);
}

fn print_summary <T: Summary>(input: T) {
    println!("Summary is: {}", input.summarize());
}