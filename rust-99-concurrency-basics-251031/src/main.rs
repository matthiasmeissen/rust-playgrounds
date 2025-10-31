use std::thread;
use std::time;

fn main() {
    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn( move || {
        let mut iter = v.iter();
        for i in 0..10 {
            println!("From seperate thread: {i}");
            let result = iter.next();
            match result {
                Some(v) => println!("Value at index {i} is: {v}"),
                None => println!("Now value at index {i}"),
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    for i in 0..5 {
        println!("From main thread: {i}");
        thread::sleep(time::Duration::from_millis(100));
    }

    handle.join().unwrap();
}