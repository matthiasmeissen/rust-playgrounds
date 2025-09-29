
struct Article {
    headline: String,
    subline: String,
    body: String,
}

struct Social {
    author: String,
    caption: String,
    shared: bool,
}


fn main() {
    let post = Social{
        author: String::from("Some Author"),
        caption: String::from("This is a post"),
        shared: false,
    };
}
