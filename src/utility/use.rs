// fn get_input<T>() -> T
pub fn get_input<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        print!("{prompt}");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut buf = String::new();
        if std::io::stdin()
            .read_line(&mut buf)
            .is_err()
        {
            println!("Failed to read input, try again.");
            continue;
        }

        match buf
            .trim()
            .parse::<T>()
        {
            Ok(value) => return value,
            Err(e) => println!("Invalid input ({e}), please re-enter."),
        }
    }
}
