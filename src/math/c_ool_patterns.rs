use rug::{Float, Rational};

pub fn c_ool_patterns() {
    let prec = 700; // ~700 bits ≈ 210 decimal digits

    let values = [
        ("1/99801", Rational::from((1, 99801))),
        ("1/99^2", Rational::from((1, 99 * 99))),
        ("1/81", Rational::from((1, 81))),
        ("1/9", Rational::from((1, 9))),
    ];

    for (name, r) in values {
        let f = Float::with_val(prec, &r);
        println!("{name} = {:.200}", f);
    }
}
