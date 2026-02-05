use std::sync::{Arc, RwLock};

/// 演示 std::sync::RwLock 不支持原子降级，只能通过释放写锁后再获取读锁来实现
fn demo_lock_release_and_reacquire() {
    let lock = Arc::new(RwLock::new(42));
    let lock_clone = Arc::clone(&lock);

    // 1. 持有并使用写锁
    {
        let mut write_guard = lock.write().unwrap();
        *write_guard = 100;
        println!("写锁: 已将数据修改为 {}", *write_guard);
        // 此处无法直接获得读锁，必须先 drop(write_guard)
    }

    // 2. 释放写锁后，获取读锁
    {
        let read_guard = lock_clone.read().unwrap();
        println!("读锁: 成功读取修改后的值 {}", *read_guard);
    }
}

fn main() {
    println!("=== RwLock<T> 锁的释放与重新获取示例 ===");
    demo_lock_release_and_reacquire();
}