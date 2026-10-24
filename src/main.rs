fn main() {
    let numbers = Vec::from_iter(0..=3);
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("length {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
    println!("sum: {}", numbers.iter().sum::<usize>());
}
