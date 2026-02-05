use std::thread;

/// 示例 10: 函数指针的线程安全性
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn example_function_pointer() {
    // 函数指针是 Send 如果它捕获的数据是 Send (全局函数不捕获，所以通常是 Send)
    let func = is_even;

    let handle = thread::spawn(move || {
        println!("10 是偶数吗? {}", func(10));
    });

    handle.join().unwrap();
}

/// 示例 11: 闭包的线程安全性
fn example_closure_thread_safety() {
    let data = [1, 2, 3, 4, 5];

    // 闭包捕获数据，move 转移所有权
    // 因为数组 [i32; 5] 是 Send，所以 move 闭包也是 Send
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("闭包计算和: {}", sum);
    });

    handle.join().unwrap();
}

fn main() {
    println!("=== 函数与闭包的线程安全性 ===");

    println!("
1. Function Pointers:");
    example_function_pointer();

    println!("
2. Closures:");
    example_closure_thread_safety();
}
