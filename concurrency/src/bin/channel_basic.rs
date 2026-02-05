use std::sync::mpsc;
use std::thread;

fn main() {
    println!("=== 基础消息传递 ===");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("你好").unwrap();
    });
    println!("接收: {}", rx.recv().unwrap());
}
