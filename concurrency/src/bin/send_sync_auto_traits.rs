use std::thread;

/// 示例 7: 类型自动推导规则
struct MyStruct {
    x: i32,
    y: String,
}

fn example_auto_derive() {
    // MyStruct 自动实现 Send 和 Sync
    // 因为它的所有字段都是 Send 和 Sync
    let data = MyStruct {
        x: 10,
        y: String::from("hello"),
    };

    let handle = thread::spawn(move || {
        println!("MyStruct 是 Send: {}, {}", data.x, data.y);
    });

    handle.join().unwrap();
}

/// 示例 8: 常见类型的 Send/Sync 状态
fn example_type_status() {
    println!("常见类型的 Send/Sync 状态:");
    println!("  i32: Send: 是, Sync: 是");
    println!("  String: Send: 是, Sync: 是");
    println!("  Vec<T>: Send: 如果 T:Send, Sync: 如果 T:Sync");
    println!("  Box<T>: Send: 如果 T:Send, Sync: 如果 T:Sync");
    println!("  Rc<T>: Send: 否, Sync: 否");
    println!("  Arc<T>: Send: 如果 T:Send, Sync: 如果 T:Send+Sync");
    println!("  Mutex<T>: Send: 如果 T:Send, Sync: 如果 T:Send");
    println!("  *mut T: Send: 否, Sync: 否");
    println!("  unsafe fn(): Send: 否, Sync: 否");
}

fn main() {
    println!("=== Send 和 Sync 自动推导与类型状态 ===");

    println!("
1. Auto Derivation Rules:");
    example_auto_derive();

    println!("
2. Common Types Status:");
    example_type_status();
}
