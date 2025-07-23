fn main() {
    let mut x = 0;
    // A loop executes over and over 
    // Until you tell it explixitly to stop with the break keyword
    loop {
        x += 1;
        if x >= 5 { break };
    }
    println!("The value of x is {x}");

    let mut counter = 0;
    // You an return a value from a loop and assign it to a variable
    // By writing the variable you want to return after the break keyword
    let result = loop {
        counter += 1;
        if counter >= 20 { break counter };
    };
    println!("The value of counter is {result}");

    // You can give a name to a loop with 'loop_name
    // This lets you specify a loop when using the break keyword
    // Which is helpful to exit an outer loop while you are inside another
    // Without the name it will exit the loop it is currently in
    let mut y = 0;
    'outer: loop {
        y = (y + 1) * 2;
        loop {
            y += x + 10;
            if y > 8000 { break 'outer}
        }
    }
    println!("The value of y is {y}");
}