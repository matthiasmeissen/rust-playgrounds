// This defines a trait
// It includes only one function with a specific signature, but no body
// All types that want to implement this trait need to include that function
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// To add a trait to a type you use an implementation block
// But in contrast to a normal one, you add: TraitName for
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}, {}", self.headline, self.location, self.author)
    }
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("Some Username"),
        content: String::from("Look at all those things I pretend to do."),
        reply: false,
        repost: false,
    };

    println!("New post: {}", post.summarize());

    print_summary(post);
}

// This function can use any type that implements the Summary trait as an input 
fn print_summary<T: Summary>(text: T) {
    println!("The summary is: {}", text.summarize());
}