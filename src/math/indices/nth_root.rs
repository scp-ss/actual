use num_traits::Float;
/// Computes number^(1/root_power) via Newton's method.
pub fn nth_root<T: Float>(number: T, root_power: T) -> T {
    let zero = T::zero();
    let one = T::one();

    if number < zero || root_power <= zero {
        return T::nan();
    }
    if number == zero {
        return zero;
    }

    let mut guess = number.max(one); // safe starting point for any positive number
    let epsilon = T::epsilon() * T::from(100).unwrap();

    loop {
        let delta = (number / guess.powf(root_power - one) - guess) / root_power;
        guess = guess + delta;
        if delta.abs() < epsilon {
            break;
        }
    }

    guess
}
