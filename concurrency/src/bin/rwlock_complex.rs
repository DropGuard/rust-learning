use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Debug)]
struct Database {
    data: Vec<String>,
}

/// 演示 RwLock 与自定义复杂结构体的配合使用
fn demo_complex_struct_with_rwlock() {
    let db = Arc::new(RwLock::new(Database { data: vec![] }));
    let mut handles = vec![];

    // 写者添加数据
    for i in 0..3 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db = db.write().unwrap();
            db.data.push(format!("数据 {}", i));
            println!("写者: 添加数据 {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles = vec![];

    // 读者读取数据
    for i in 0..3 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let db = db.read().unwrap();
            println!("读者 {:?}: {:?}", i, db.data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("=== RwLock<T> 与复杂类型示例 ===");
    demo_complex_struct_with_rwlock();
}