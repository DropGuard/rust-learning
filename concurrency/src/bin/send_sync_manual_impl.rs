use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::thread;

/// 示例 4: 手动实现 Send (unsafe)
struct SafeSendData {
    data: i32,
}

// 安全条件: SafeSendData 不包含任何非 Send 的内部类型
unsafe impl Send for SafeSendData {}

fn example_manual_send() {
    let data = SafeSendData { data: 42 };

    let handle = thread::spawn(move || {
        println!("手动实现的 Send: {}", data.data);
    });

    handle.join().unwrap();
}

/// 示例 5: 手动实现 Sync (unsafe)
struct SafeSyncData {
    data: Mutex<i32>,
}

// 安全条件: &SafeSyncData 可以安全地跨线程共享
// 因为 Mutex<T> 是 Sync，当 T: Send 时
unsafe impl Sync for SafeSyncData {}

fn example_manual_sync() {
    let data = Arc::new(SafeSyncData {
        data: Mutex::new(0),
    });

    let mut handles = vec![];

    for _ in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("计数: {}", *data.data.lock().unwrap());
}

/// 示例 6: PhantomData 用于实现 Send/Sync
struct Wrapper<T> {
    data: T,
    _marker: PhantomData<*const ()>, // *const () 不是 Send/Sync
}

// 如果我们想要让 Wrapper 只有在某些条件下才是 Send/Sync
unsafe impl<T: Send> Send for Wrapper<T> {}

fn example_phantomdata() {
    let data = Wrapper {
        data: 42,
        _marker: PhantomData,
    };

    let handle = thread::spawn(move || {
        println!("Wrapper with i32 (via PhantomData): {}", data.data);
    });

    handle.join().unwrap();
}

/// 示例 13: 自定义类型的条件 Send/Sync 实现
struct ConditionalSend<T> {
    data: T,
}

// 只有当 T: Send 时，ConditionalSend<T> 才是 Send
unsafe impl<T: Send> Send for ConditionalSend<T> {}

// 只有当 T: Sync 时，ConditionalSend<T> 才是 Sync
unsafe impl<T: Sync> Sync for ConditionalSend<T> {}

fn example_conditional_send_sync() {
    // i32 是 Send，所以 ConditionalSend<i32> 是 Send
    let data = ConditionalSend { data: 42 };

    let handle = thread::spawn(move || {
        println!("ConditionalSend<i32>: {}", data.data);
    });

    handle.join().unwrap();

    // Arc<ConditionalSend<i32>> 是 Sync
    let data_arc = Arc::new(ConditionalSend { data: 42 });
    let data_clone = Arc::clone(&data_arc);

    let handle_sync = thread::spawn(move || {
        println!("Arc<ConditionalSend<i32>>: {}", data_clone.data);
    });

    handle_sync.join().unwrap();
}

fn main() {
    println!("=== Send 和 Sync 手动实现 ===");

    println!("
1. Manual Send Implementation:");
    example_manual_send();

    println!("
2. Manual Sync Implementation:");
    example_manual_sync();

    println!("
3. PhantomData Usage:");
    example_phantomdata();

    println!("
4. Conditional Send/Sync:");
    example_conditional_send_sync();
}
