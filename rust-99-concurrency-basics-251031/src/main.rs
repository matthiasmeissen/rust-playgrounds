
use std::thread;
use std::time;

fn main() {
    let v = vec![1, 2, 3, 4];

    thread::spawn(|| {
        for i in 0..10 {
            println!("From seperate thread: {i}");
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    for i in 0..5 {
        println!("From main thread: {i}");
        thread::sleep(time::Duration::from_millis(200));
    }
}
