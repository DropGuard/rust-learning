use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

/// 示例 9: 组合智能指针的线程安全性
fn example_smart_pointers_thread_safety() {
    // Arc<Mutex<T>>: Send if T:Send, Sync if T:Send
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Arc<Mutex<i32>> 计数: {}", *counter.lock().unwrap());
}

/// 示例 12: 原子类型的 Send/Sync
fn example_atomic_types() {
    // 原子类型都是 Send 和 Sync
    let atomic = Arc::new(AtomicI32::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let atomic = Arc::clone(&atomic);
        let handle = thread::spawn(move || {
            atomic.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("AtomicI32 是 Send 和 Sync: {}", atomic.load(Ordering::SeqCst));
}

fn main() {
    println!("=== 线程安全容器与原子类型 ===");

    println!("
1. Arc<Mutex<T>> Usage:");
    example_smart_pointers_thread_safety();

    println!("
2. Atomic Types:");
    example_atomic_types();
}
