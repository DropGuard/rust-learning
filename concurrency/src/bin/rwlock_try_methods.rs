use std::sync::{Arc, RwLock};

/// 演示非阻塞的 try_read 和 try_write 方法
fn demo_try_lock_methods() {
    let lock = Arc::new(RwLock::new(42));
    let lock_clone = Arc::clone(&lock);

    // 持有写锁
    let _write_guard = lock.write().unwrap();

    // 尝试读锁（会失败）
    let result = lock_clone.try_read();
    match result {
        Ok(_guard) => println!("获取读锁成功"),
        Err(_) => println!("无法获取读锁（写锁被持有）"),
    }

    // 尝试写锁（会失败）
    let result = lock_clone.try_write();
    match result {
        Ok(_guard) => println!("获取写锁成功"),
        Err(_) => println!("无法获取写锁（写锁已被持有）"),
    }
}

fn main() {
    println!("=== RwLock<T> try_read 和 try_write 示例 ===");
    demo_try_lock_methods();
}