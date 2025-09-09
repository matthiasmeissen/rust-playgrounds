
enum SpreadsheetCell {
    Int(i32), 
    Float(f32), 
    Text(String)
}

fn main() {
    let cell1 = SpreadsheetCell::Int(20);
}
