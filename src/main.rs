fn main() {
    let t1 = std::thread::spawn(f);
    let t2 = std::thread::spawn(f);

    println!("Hello from main thread!");

    if let Err(err) = t1.join() {
        eprintln!("{err:#?}");
    }

    if let Err(err) = t2.join() {
        eprintln!("{err:#?}");
    }
}

fn f() {
    // Calling this function may be more efficient than accessing the current
    // thread id through the current thread handle. i.e. `thread::current().id()`.
    let id = std::thread::current().id();
    println!("Hello from {id:?} thread!");
}
