use std::sync::{Arc, RwLock};
use std::thread;

/// 演示 RwLock 的毒化机制（当持有写锁的线程 panic 时）
fn demo_rwlock_poisoning() {
    let lock = Arc::new(RwLock::new(42));
    let lock_clone = Arc::clone(&lock);

    // 1. 创建一个会崩溃的写者
    let handle = thread::spawn(move || {
        let mut data = lock_clone.write().unwrap();
        *data = 100;
        println!("写者: 已修改数据，现在准备 panic 以毒化锁...");
        panic!("意料之中的 panic！");
    });

    // 等待该写者结束
    let _ = handle.join();

    // 2. 尝试在毒化后获取读锁
    let result = lock.read();
    match result {
        Ok(guard) => println!("读取成功: {}", *guard),
        Err(e) => {
            println!("读锁获取失败，锁已被毒化: {:?}", e);
            // 依然可以通过 into_inner 强制获取数据
            let recovered = e.into_inner();
            println!("强制恢复的数据值为: {}", *recovered);
        }
    }
}

fn main() {
    println!("=== RwLock<T> 毒化 (Poisoning) 示例 ===");
    demo_rwlock_poisoning();
}