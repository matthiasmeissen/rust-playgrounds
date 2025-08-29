#[derive (Debug)]
enum Shape {
    Rectangle{w: u32, h: u32},
    Cirle{r: u32},
}

// There is a built in enum type called Option<T> (part of the std library and brought into scope automatically)
// It has two variants: None and Some(T)
// It states that either a value is present, or it is not present
// It is a more safe implementation of the NULL concept from other language

// The value of the Option<T> enum is that with any other enum
// In order to access T you need to define all possible cases (using match control logic)
// This means you would have to write what happens when T present and when it is not present
// This makes it very robust and helps eliminate NULL realeted errors

fn main() {
    // This uses the Option enum with a u32 type
    // The some_number is holding the variant Some() with value 4
    // the absent_number is holding the variant None
    let some_number: Option<u32> = Some(4);
    let absent_number: Option<u32> = None;

    let x: u32 = 8;
    let y: Option<u32> = Some(4);

    // This line wont work, since x and y a two different types
    // x is of type u32 and y is of type Option<u32> so they cant be added together
    //let sum = x + y;

    println!(
        "Some number is {:?} and absent number is {:?}",
        some_number, absent_number
    );
    
    let shape1 = Shape::Rectangle{w: 40, h: 20};
    println!("shape1 is: {:?}", shape1);
}