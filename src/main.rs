use std::{sync::Mutex, thread, time::Duration};

fn main() {
    let n = Mutex::new(0u32);
    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let Ok(mut guard) = n.lock() else {
                    return;
                };
                for _ in 0..100 {
                    *guard = (*guard).wrapping_add(1u32);
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    if let Ok(res) = n.into_inner() {
        assert_eq!(res, 1000);
    }
}
