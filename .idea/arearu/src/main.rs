use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(||{
        for i in 1..10{
            println!("Hello, from thread{}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    for i in 1..30{
        println!("Hello from main thread! {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}
