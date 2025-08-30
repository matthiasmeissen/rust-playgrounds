
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin1 = Coin::Penny;
    println!("The value in cents of coin1 is {}", value_in_cents(coin1));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
