use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let x = 50;
 let h1 =  thread::spawn(|| {
    for i in 1..20{
        println!("Hello from spawned thread! {}", i);
        thread::sleep(Duration::from_secs(1)); 

    }
   });
    for i in 1..=30{
        println!("Hello from main thread! {}", i);
        thread::sleep(Duration::from_secs(1));
    }
  let mut k = Arc::new(Mutex::new(0));
  let val = k.clone();
  let h2 =  thread::spawn(move||{
        for i in 1..=x{
            let mut k_guard = val.lock().unwrap();
            *k_guard += i;
            println!("Hello from moved spawned thread! {}", i);
            thread::sleep(Duration::from_secs(1));
        }

    });
    h1.join().unwrap();
    h2.join().unwrap();
    println!("Final value of k: {}", *k.lock().unwrap());
}
