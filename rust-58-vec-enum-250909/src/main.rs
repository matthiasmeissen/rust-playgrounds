// The associated data of an enum variant can be defined in different ways
enum Message {
    // No data associated
    Quit,
    // Data in named fiels, like a struct
    Move {x: i32, y: i32},
    // One data value
    Write(String),
    // Multiple data values
    Color(u8, u8, u8),
}

enum SpreadsheetCell {
    Int(i32), 
    Float(f32), 
    Text(String)
}

impl SpreadsheetCell {
    fn get_value(&self) {
        match self {
            Self::Int(i) => println!("Type: Int, Value: {i}"),
            Self::Float(i) => println!("Type: Float, Value: {i}"),
            Self::Text(i) => println!("Type: String, Value: {i}")
        }
    }
}

fn main() {
    let cell1 = SpreadsheetCell::Int(20);
    let cell2 = SpreadsheetCell::Float(0.4);
    let cell3 = SpreadsheetCell::Text(String::from("Hello people"));

    let mut spreadsheet: Vec<SpreadsheetCell> = Vec::new();
    spreadsheet.push(cell1);
    spreadsheet.push(cell2);
    spreadsheet.push(cell3);

    for cell in &spreadsheet {
        cell.get_value();
    };
}