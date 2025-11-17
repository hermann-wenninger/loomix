use std::thread;
use std::time::Duration;

fn main() {
   thread::spawn(|| {
    for i in 1..20{
        println!("Hello from spawned thread! {}", i);
        thread::sleep(Duration::from_secs(1));  
    }
   })
    for i in 1..=30{
        println!("Hello from main thread! {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}
