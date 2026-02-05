use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

/// 演示读者和写者并发竞争锁的情况
fn demo_readers_and_writers() {
    let lock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 创建读者
    for i in 0..3 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            for _ in 0..3 {
                let data = lock.read().unwrap();
                println!("读者 {:?}: 读取 {}", i, *data);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    // 创建写者
    for i in 0..2 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            for _j in 0..2 {
                let mut data = lock.write().unwrap();
                *data += 10;
                println!("写者 {:?}: 写入 {}", i, *data);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终值: {}", *lock.read().unwrap());
}

fn main() {
    println!("=== RwLock<T> 读者与写者示例 ===");
    demo_readers_and_writers();
}