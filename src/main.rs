fn main() {
    let numbers = vec![1, 2, 3];
    if let Err(err) = std::thread::spawn(move || {
        for num in &numbers {
            println!("{num}");
        }
    })
    .join()
    {
        eprintln!("{err:?}");
    }
}
