use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

/// 演示写者可能遭遇饥饿的场景（读者持续占用锁）
fn demo_writer_starvation() {
    let lock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 1. 启动多个持续占用读锁的读者
    for i in 0..5 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            for _ in 0..50 {
                let _guard = lock.read().unwrap();
                thread::sleep(Duration::from_millis(2));
            }
            println!("读者 {} 完成使命", i);
        });
        handles.push(handle);
    }

    // 2. 启动一个尝试获取写锁的写者
    let lock_clone = Arc::clone(&lock);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5)); // 确保读者先运行
        println!("写者: 开始排队尝试获取写锁...");
        let mut guard = lock_clone.write().unwrap();
        println!("写者: 历经磨难，终于获取写锁成功！");
        *guard = 999;
        thread::sleep(Duration::from_millis(50));
        println!("写者: 写入完成并释放锁");
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("主线程: 最终检测值为 {}", *lock.read().unwrap());
}

fn main() {
    println!("=== RwLock<T> 写者饥饿问题演示 ===");
    demo_writer_starvation();
}