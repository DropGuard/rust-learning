use std::sync::{Arc, RwLock};
use std::thread;

/// 模拟一个简单的基于 RwLock 的并发安全缓存
struct SimpleCache {
    data: RwLock<Vec<(String, String)>>,
}

impl SimpleCache {
    fn new() -> Self {
        Self {
            data: RwLock::new(vec![]),
        }
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.push((key, value));
    }

    fn list_all(&self) -> Vec<(String, String)> {
        let data = self.data.read().unwrap();
        data.clone()
    }
}

/// 演示缓存系统的并发读写
fn demo_cache_system() {
    let cache = Arc::new(SimpleCache::new());
    let mut handles = vec![];

    // 1. 多个并发写者注入数据
    for i in 0..3 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let key = format!("k_{}_{}", i, j);
                let value = format!("v_{}_{}", i, j);
                cache.set(key, value);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // 2. 多个并发读者读取数据
    let mut handles = vec![];
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let items = cache.list_all();
            println!("读者 {}: 当前获取到 {} 个缓存条目", i, items.len());
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终缓存完整列表: {:?}", cache.list_all());
}

fn main() {
    println!("=== 实际应用场景: RwLock 实现的缓存系统 ===");
    demo_cache_system();
}