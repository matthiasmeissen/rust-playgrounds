
fn main() {
    let novel = String::from("Once upon a time. There was a planet called earth.");
    // The split() method returns an iterator define by the input character (In this case The first and the second sentence)
    let mut sentences = novel.split('.');
    // The next() method returns an element of an iterator
    // It has some sort of internal counter, so when called first it returns index 0 
    // when called again it returns index 1 and so on
    let first_sentence = sentences.next().unwrap();
    let second_sentence = sentences.next().unwrap();

    println!("{}, {}", first_sentence, second_sentence);
}
