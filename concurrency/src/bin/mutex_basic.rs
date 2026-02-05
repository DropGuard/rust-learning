use std::sync::Mutex;

/// 演示 Mutex 的基本加锁、修改和自动释放 (RAII)
fn basic_lock_usage() {
    println!("--- 基础加锁与 RAII ---");
    let m = Mutex::new(5);

    {
        // 获取锁，返回 MutexGuard
        // MutexGuard 是一个智能指针，实现了 Deref 和 DerefMut
        let mut guard = m.lock().unwrap();
        *guard = 6;
        println!("修改后: {}", *guard);
    } // MutexGuard 在此处被 drop，锁自动释放

    // 锁释放后可以再次获取
    println!("最终值: {}", *m.lock().unwrap());
}

/// 演示 MutexGuard 如何通过 Deref 特性透明地调用内部类型的方法
fn mutex_guard_deref() {
    println!("\n--- MutexGuard 的 Deref 特性 ---");
    let mutex_s = Mutex::new(String::from("hello"));

    {
        // 1. Deref: 直接调用 String 的方法
        let guard = mutex_s.lock().unwrap();
        println!("字符串长度: {}", guard.len());
    }

    // 2. DerefMut: 修改内部 String
    let mut guard = mutex_s.lock().unwrap();
    guard.push_str(" world");
    println!("修改后的字符串: {}", *guard);
}

fn main() {
    println!("=== Mutex 基础示例 ===");
    basic_lock_usage();
    mutex_guard_deref();
}
