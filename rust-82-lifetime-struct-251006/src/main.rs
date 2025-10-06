// This struct has a generic lifetime called a
// In the part member of the struct we assign this lifetime to the string slice (which si a reference)
// This means that an instance of the ImportantExpert struct can not outlive the reference to the string slice
struct ImportantExcerpt <'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Once upon a time. There was a planet called earth.");
    // The split() method returns an iterator define by the input character (In this case The first and the second sentence)
    let mut sentences = novel.split('.');
    // The next() method returns an element of an iterator
    // It has some sort of internal counter, so when called first it returns index 0 
    // when called again it returns index 1 and so on
    let first_sentence = sentences.next().unwrap();
    let second_sentence = sentences.next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence
    };

    println!("The first sentence is: {}", i.part);

    // The trim() method removes any leading or trailing whitespace and linebreak characters
    println!("First: {} \nSecond: {}", first_sentence.trim(), second_sentence.trim());
}