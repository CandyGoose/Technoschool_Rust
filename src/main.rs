use bigdecimal::{BigDecimal};
use num_bigint::BigInt;
use std::str::FromStr;

fn arithmetic_operations<T>(a: T, b: T)
where
    T: std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + std::ops::Mul<Output = T>
    + std::ops::Div<Output = T>
    + PartialEq
    + From<i32>
    + std::fmt::Display
    + Clone,
{
    println!("{} + {} = {}", a.clone(), b.clone(), a.clone() + b.clone());
    println!("{} - {} = {}", a.clone(), b.clone(), a.clone() - b.clone());
    println!("{} * {} = {}", a.clone(), b.clone(), a.clone() * b.clone());

    if b != T::from(0) {
        println!("{} / {} = {}", a.clone(), b.clone(), a.clone() / b.clone());
    } else {
        println!("Division by zero is not allowed!");
    }
}

fn main() {
    let a_float = BigDecimal::from_str("1000000000000000000000.123456789").unwrap();
    let b_float = BigDecimal::from_str("2000000000000000000000.987654321").unwrap();

    arithmetic_operations(a_float.clone(), b_float.clone());

    let a_int = BigInt::from_str("1000000000000000000").unwrap();
    let b_int = BigInt::from_str("2000000000000000000").unwrap();

    let a_int_as_float = BigDecimal::from(a_int);
    let b_int_as_float = BigDecimal::from(b_int);

    arithmetic_operations(a_int_as_float, b_int_as_float);
}
