
fn main() {
    let my_string = ref_inside();
}

fn ref_inside() -> &String {
    let s = String::from("Hello");
    &s
}
