use malachite::num::arithmetic::traits::Pow;
use malachite::num::conversion::string::options::ToSciOptions;
use malachite::num::conversion::traits::ToSci;
use malachite::Rational;

fn main() {
    let maximum_n = 100;
    let mut options = ToSciOptions::default();
    options.set_precision(30);

    for value in 1..=maximum_n {
        println!(
            "{} : {:0<31}",
            value,
            format!("{}", expected_highest(value).to_sci_with_options(options))
        );
    }
}

fn expected_highest(value: u64) -> malachite::Rational {
    let summation: Rational = (1..=6)
        .map(|i: u64| (Rational::from(i) / Rational::from(6)).pow(value))
        .sum();

    Rational::from(7) - summation
}
