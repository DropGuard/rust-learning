use std::rc::Rc;
use std::sync::Arc;
use std::thread;

/// 示例 1: Send trait 基本概念
/// Send: 类型可以跨线程转移所有权
fn example_send_trait() {
    let data = String::from("hello");

    // String 是 Send，可以移动到新线程
    let handle = thread::spawn(move || {
        println!("线程接收到: {}", data);
    });

    handle.join().unwrap();
}

/// 示例 2: 非 Send 类型 (Rc)
/// Rc 不是 Send，因为它使用非原子引用计数
fn example_non_send() {
    let _data = Rc::new(42);

    // 以下代码会编译错误
    /*
    let handle = thread::spawn(move || {
        println!("{}", _data);
    });
    handle.join().unwrap();
    */

    println!("Rc 不是 Send，不能在线程间转移");
    println!("解决方案: 使用 Arc（原子引用计数）");

    let arc_data = Arc::new(42);
    let handle = thread::spawn(move || {
        println!("Arc 是 Send: {}", arc_data);
    });
    handle.join().unwrap();
}

/// 示例 3: Sync trait 基本概念
/// Sync: 类型的引用可以跨线程共享
fn example_sync_trait() {
    let data = Arc::new(42);

    // Arc<i32> 是 Sync，可以在线程间共享引用
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    let handle1 = thread::spawn(move || {
        println!("线程1: {}", data1);
    });

    let handle2 = thread::spawn(move || {
        println!("线程2: {}", data2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    println!("=== Send 和 Sync 基本概念 ===");
    
    println!("
1. Send Trait:");
    example_send_trait();

    println!("
2. Non-Send Type (Rc):");
    example_non_send();

    println!("
3. Sync Trait:");
    example_sync_trait();
}
