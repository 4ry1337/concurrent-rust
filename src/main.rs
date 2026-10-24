use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num = AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100usize {
                let processed = i.wrapping_add(1);
                thread::sleep(Duration::from_millis(10));
                num.store(processed, std::sync::atomic::Ordering::Relaxed);
            }
        });
        loop {
            let n = num.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working {n}/100");
            thread::sleep(Duration::from_millis(10));
        }
    });

    println!("Done!");
}
