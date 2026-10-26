use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..25 {
                    thread::sleep(Duration::from_millis(10));
                    // compare_exchange_weak may fail spuriously (return Err even though the value
                    // still equals `current`). That's fine here: we are already in a retry loop,
                    // so a spurious failure just costs one extra iteration.
                    // Tradeoff: on LL/SC CPUs (ARM, RISC-V) weak avoids the hidden retry loop that
                    // strong compare_exchange needs, so it's cheaper inside a loop. On x86 both are
                    // the same `lock cmpxchg`. Without a loop (a one-shot attempt) use the strong
                    // version, otherwise a spurious Err would look like another thread won.
                    let mut current = num.load(std::sync::atomic::Ordering::Relaxed);
                    loop {
                        let new = current.wrapping_add(1);
                        match num.compare_exchange_weak(
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
