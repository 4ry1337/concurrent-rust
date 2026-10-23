fn main() {
    let numbers = Vec::from_iter(0..=100);
    let average =
        match std::thread::spawn(move || numbers.iter().sum::<usize>().checked_div(numbers.len()))
            .join()
        {
            Ok(Some(res)) => res,
            Ok(None) => {
                eprintln!("overflow or empty input");
                return;
            }
            Err(err) => {
                eprintln!("{err:?}");
                return;
            }
        };
    println!("average: {average}");
}
