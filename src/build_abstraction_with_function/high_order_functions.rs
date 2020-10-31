extern crate num;

use num::{Num, PrimInt, Unsigned};
use std::iter::Product;

fn factorial<T>(n: T) -> T
where
  T: PrimInt + Unsigned + Product,
{
  num::range(T::one(), n + T::one()).product()
}

// generic function that can take any kind of number
fn cube<T>(n: T) -> T
where
  T: Num + Copy,
{
  n * n * n
}

#[test]
fn high_order_functions_tests() {
  println!("factorial of {}", factorial(3_u8));
  println!("factorial of {}", factorial(3_u16));
  println!("factorial of {}", factorial(3_u32));
  println!("factorial of {}", factorial(3_u64));
  println!("cube of {}", cube(3_f64));
  println!("cube of {}", cube(3_i128));
  println!("cube of {}", cube(3.000001));
}
