use std::thread;

/// 示例 14: 检查类型是否实现 Send/Sync
fn example_check_send_sync() {
    // 使用 trait bound 约束来检查
    fn is_send<T: Send>(_value: &T) {
        println!("类型实现了 Send");
    }

    fn is_sync<T: Sync>(_value: &T) {
        println!("类型实现了 Sync");
    }

    let data = 42;
    is_send(&data);
    is_sync(&data);
}

/// 示例 15: 使用 where 子句约束线程安全性
fn example_where_clause() {
    fn spawn_with_check<T: Send + 'static>(_data: T) {
        let _handle = thread::spawn(move || {
            // data 在这里可以使用
        });
        println!("成功生成带 Send 约束的线程");
    }

    let data_str = String::from("hello");
    spawn_with_check(data_str);

    let data_vec = vec![1, 2, 3];
    spawn_with_check(data_vec);
}

fn main() {
    println!("=== Send 和 Sync 的约束检查 ===");

    println!("
1. Explicit Checks via Generic Bounds:");
    example_check_send_sync();

    println!("
2. Thread Spawning with 'where' Clauses:");
    example_where_clause();
}
