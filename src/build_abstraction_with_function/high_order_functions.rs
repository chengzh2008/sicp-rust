extern crate num;

use crate::build_abstraction_with_function::functions_and_their_processes::inc;

use num::{Num, PrimInt, Unsigned};
use std::iter::Product;

pub fn factorial<T>(n: T) -> T
where
  T: PrimInt + Unsigned + Product,
{
  num::range(T::one(), n + T::one()).product()
}

// generic function that can take any kind of number
pub fn cube<T>(n: T) -> T
where
  T: Num + Copy,
{
  n * n * n
}

fn sum_integers(a: i128, b: i128) -> i128 {
  if a > b {
    0
  } else {
    a + sum_integers(a + 1, b)
  }
}

fn sum_cube(a: i128, b: i128) -> i128 {
  if a > b {
    0
  } else {
    cube(a) + sum_cube(a + 1, b)
  }
}

fn pi_sum(a: i128, b: i128) -> f64 {
  if a > b {
    0.0
  } else {
    1.0 / (a * (a + 2)) as f64 + pi_sum(a + 4, b)
  }
}

pub fn sum(term: fn(i: i128) -> f64, a: i128, next: fn(i: i128) -> i128, b: i128) -> f64 {
  if a > b {
    0.0
  } else {
    term(a) + sum(term, next(a), next, b)
  }
}

pub fn sum_f(term: impl Fn(f64) -> f64, a: f64, next: impl Fn(f64) -> f64, b: f64) -> f64 {
  if a > b {
    0.0
  } else {
    term(a) + sum_f(term, next(a), next, b)
  }
}

fn integral(f: impl Fn(f64) -> f64, a: f64, b: f64, dx: f64) -> f64 {
  sum_f(f, a, |x| x + dx, b) * dx
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
  println!("sum cube of 1..10 {}", sum_cube(1, 10));
  println!(
    "sum of cube of 1..10 {}",
    sum(|i| (i * i * i) as f64, 1, inc, 10)
  );
  println!("sum of 1..10 {}", sum_integers(1, 10));
  println!("pi simulation {}", 8.0 * pi_sum(1, 1000));
  println!(
    "pi simulation {}",
    8.0 * sum(|i| 1.0 / (i * (i + 2)) as f64, 1, |i| i + 4, 1000)
  );

  // simulation of 1/4 calculation
  println!(
    "integral of cube from 0 to 1 {}",
    integral(cube, 0.0, 1.0, 0.01)
  );
  println!(
    "integral of cube from 0 to 1 {}",
    integral(cube, 0.0, 1.0, 0.001)
  );
}
