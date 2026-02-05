use std::sync::{Arc, Mutex};
use std::thread;

/// 优雅的封装示例 (Newtype Pattern)
///
/// 特点：
/// 1. 结构体直接持有 Arc。
/// 2. derive Clone：克隆结构体 = 克隆 Arc 指针（成本极低）。
/// 3. 对外隐藏 Mutex：无需 lock/unwrap，直接调方法。
#[derive(Clone)]
struct Counter {
    // 内部状态被 Arc 包裹，实现多线程共享
    inner: Arc<Mutex<i32>>,
}

impl Counter {
    fn new(initial: i32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(initial)),
        }
    }

    fn increment(&self) {
        // 内部可变性 (Interior Mutability)
        // Mutex 允许通过不可变引用 (&self) 来获取内部数据的可变引用。
        let mut count = self.inner.lock().unwrap();
        *count += 1;
    }

    fn get(&self) -> i32 {
        *self.inner.lock().unwrap()
    }
}

fn main() {
    println!("=== Mutex 优雅封装示例 ===");

    let counter = Counter::new(0);
    let mut handles = vec![];

    for i in 0..10 {
        // 直接 clone，获取计数器的另一个句柄
        let c = counter.clone();

        let handle = thread::spawn(move || {
            c.increment();
            // <-----此处锁已释放
            println!("线程 {}  +1，当前结果 {}", i, c.get());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终结果: {}", counter.get());
}
