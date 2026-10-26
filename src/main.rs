use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..25 {
                    thread::sleep(Duration::from_millis(10));
                    let mut current = num.load(std::sync::atomic::Ordering::Relaxed);
                    loop {
                        let new = current.wrapping_add(1);
                        match num.compare_exchange(
                            current,
                            new,
                            std::sync::atomic::Ordering::Relaxed,
                            std::sync::atomic::Ordering::Relaxed,
                        ) {
                            Ok(_) => break,
                            Err(v) => current = v,
                        }
                    }
                }
            });
        }
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
