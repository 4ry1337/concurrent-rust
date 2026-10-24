fn main() {
    // static
    static A: [i32; 2] = [1, 2];
    // release ownership, promsising to never drop it
    // we are sharing pointer to value
    // tradeoff: memory leak
    let b: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    // reference counting
    let c = std::sync::Arc::new([1, 2, 3]);
    let d = c.clone();
    std::thread::scope(|s| {
        s.spawn(|| dbg!(A[0]));
        s.spawn(|| dbg!(A[1]));
        s.spawn(move || dbg!(b[0]));
        s.spawn(move || dbg!(b[1]));
        s.spawn(move || dbg!(b[2]));
        // s.spawn(move || dbg!(c));
        s.spawn({
            let c = c.clone();
            move || {
                dbg!(c);
            }
        });
        s.spawn(move || dbg!(d));
    });
}
