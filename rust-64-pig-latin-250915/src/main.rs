fn main() {
    let word1 = String::from("first");
    let word2 = String::from("apple");

    println!("{} becomes {}", word1, pig_latin(&word1[..]));
    println!("{} becomes {}", word2, pig_latin(&word2[..]));
}

fn pig_latin(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut chars = word.chars();

    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
            format!("{}-hay", word)
        }
        _ => {
            let rest = chars.as_str();
            format!("{}-{}ay", rest, first_char)
        }
    }

}

fn pig_latin1(word: &str) -> String {
    let mut new_word = String::from(word);

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for vowel in vowels {
        if word.starts_with(vowel) {
            new_word.push_str("-hay");
            return new_word;
        } 
    }

    let first_character = &new_word.clone()[0..1];
    let without_first = &new_word.clone()[1..];

    new_word = String::from(without_first);
    let addition = format!("-{}ay", first_character);
    new_word.push_str(&addition);

    new_word
}