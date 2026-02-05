use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::time::Instant;

/// 比较 RwLock 和 Mutex 在高频率读操作场景下的性能差异
fn compare_rwlock_vs_mutex_performance() {
    let rwlock = Arc::new(RwLock::new(0));
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 1. RwLock 性能测试
    let start_rw = Instant::now();
    for i in 0..10 {
        let lock = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let _guard = lock.read().unwrap();
            }
            println!("RwLock 读者 {} 完成", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let rwlock_duration = start_rw.elapsed();

    // 2. Mutex 性能测试
    let mut handles = vec![];
    let start_mtx = Instant::now();
    for i in 0..10 {
        let lock = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let _guard = lock.lock().unwrap();
            }
            println!("Mutex 读者 {} 完成", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mutex_duration = start_mtx.elapsed();

    println!("\n结果对比:");
    println!("RwLock 读操作总耗时: {:?}", rwlock_duration);
    println!("Mutex 读操作总耗时: {:?}", mutex_duration);
    println!("结论: 在多读者场景下，RwLock 由于允许并发读取，性能通常优于 Mutex。");
}

fn main() {
    println!("=== RwLock<T> 性能对比示例 ===");
    compare_rwlock_vs_mutex_performance();
}