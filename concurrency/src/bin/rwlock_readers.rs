use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

/// 演示多个并发读者的情况
fn demo_multiple_readers() {
    let lock = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // 创建 5 个读者线程
    for i in 0..5 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let data = lock.read().unwrap();
            println!("读者 {:?}: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100));
            println!("读者 {:?}: 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("=== RwLock<T> 多个读者示例 ===");
    demo_multiple_readers();
    println!("所有读者完成");
}