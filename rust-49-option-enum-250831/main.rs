
fn main() {
    let num: i32 = 20;
    let num_present = plus_one(num);
    let num_absent = plus_one()
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        x::None => None,
        x::Some(i) => Some(i + 1),
    }
}
