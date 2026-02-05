use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 多生产者模式 ===");
    let (tx, rx) = mpsc::channel();
    for i in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("来自生产者{}", i)).unwrap();
            thread::sleep(Duration::from_millis(10));
        });
    }
    drop(tx); 
    for msg in rx { println!("  收到: {}", msg); }
}
