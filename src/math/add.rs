pub fn add_two<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

// pub fn add_vectors<T>(a: Vec<T>, b: T) -> T
// where
//     T: std::ops,
// {
// }

// writeln!
