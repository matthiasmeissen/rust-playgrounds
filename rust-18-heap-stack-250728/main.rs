
fn main() {
    describe_stack();
}

fn describe_stack() {
    println!("-------- Stack --------");
    println!("The stack is last in first out (It you add one thing to the end, and there is not enough space, it removes the first item)");
    println!("Like a pile of plates");
    println!("This is called pushing onto the stack and popping off the stack (a bit similar to manipulating arrays)");
    println!("All data on the stack must have a fixed size on compile time.");
    println!("<---pop---[ stack memory with fixed size items ]<---push---");
    println!("The stack is very fast, it just adds new things to the end of it.");
}

fn describe_heap() {
    println!("-------- Heap --------");
    println!("To store data on the heap you request a specific amount of space, the heap returns a pointer to that space.");
    println!("The heap stores that data at any locaton (It looks for free space and puts it there)");
    println!("This process is called memory allocation.");
    println!("Similar to beeing seated in a table at the restaurant. If one comes lat it asks the waiter where the table is.");
    println!("The heap is slower that the stack, since the allocator has to look for free memory.");
}
