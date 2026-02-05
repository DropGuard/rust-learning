use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    counter: i32,
    values: Vec<i32>,
}

/// 演示使用 Arc<Mutex<T>> 共享和修改复杂类型
fn run_complex_data_sharing() {
    println!("--- 共享复杂类型演示 ---");

    let data = Arc::new(Mutex::new(SharedData {
        counter: 0,
        values: vec![],
    }));

    let mut handles = vec![];

    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            data.counter += 1;
            data.values.push(i);
            println!(
                "线程 {}: counter={}, values={:?}",
                i, data.counter, data.values
            );
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n[结果] 最终共享数据: {:?}", *data.lock().unwrap());
}

fn main() {
    println!("=== Mutex 与 Arc 组合 ===\n");

    run_complex_data_sharing();

    println!("\n============");
}
