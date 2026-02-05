use std::sync::RwLock;

/// 演示 RwLock 的基本读写操作
fn demo_basic_operations() {
    let lock = RwLock::new(5);

    // 读操作
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap(); // 可以同时有多个读锁
        println!("读操作 1: {}", *r1);
        println!("读操作 2: {}", *r2);
    }

    // 写操作
    {
        let mut w = lock.write().unwrap();
        *w = 10;
        println!("写操作后: {}", *w);
    }

    // 再次读取
    {
        let r = lock.read().unwrap();
        println!("最终值: {}", *r);
    }
}

fn main() {
    println!("=== RwLock<T> 基本读写操作 ===");
    demo_basic_operations();
}