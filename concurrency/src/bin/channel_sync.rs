use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 同步通道 (容量为0) ===");
    let (tx, rx) = mpsc::sync_channel(0);
    thread::spawn(move || {
        println!("  [子线程] 尝试发送...");
        tx.send("同步数据").unwrap();
        println!("  [子线程] 发送成功!");
    });

    thread::sleep(Duration::from_millis(50));
    println!("主线程准备接收...");
    rx.recv().unwrap();
}
