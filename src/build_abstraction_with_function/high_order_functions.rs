extern crate num;

use crate::build_abstraction_with_function::functions_and_their_processes::{gcd, inc, is_prime};
use recur_fn::{recur_fn, RecurFn};

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

pub fn double<T>(n: T) -> T
where
  T: Num + Copy,
{
  n + n
}

pub fn square<T>(n: T) -> T
where
  T: Num + Copy,
{
  n * n
}

fn cube_fn(n: i128) -> f64 {
  (n * n * n) as f64
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

pub fn sum(term: impl Fn(i128) -> f64, a: i128, next: impl Fn(i128) -> i128, b: i128) -> f64 {
  if a > b {
    0.0
  } else {
    term(a) + sum(term, next(a), next, b)
  }
}

// pass function or closure as reference to avoid "move" compile issue
// Exercise 1.30
pub fn sum_iter(term: &impl Fn(i128) -> f64, a: i128, next: impl Fn(i128) -> i128, b: i128) -> f64 {
  let helper = recur_fn(|helper, (a, sum_state)| {
    if a > b {
      sum_state
    } else {
      helper((next(a), term(a) + sum_state))
    }
  });

  helper.call((a, 0.0))
}

pub fn product(term: impl Fn(i128) -> f64, a: i128, next: impl Fn(i128) -> i128, b: i128) -> f64 {
  if a > b {
    1.0
  } else {
    term(a) * product(term, next(a), next, b)
  }
}

pub fn product_iter(
  term: impl Fn(i128) -> f64,
  a: i128,
  next: impl Fn(i128) -> i128,
  b: i128,
) -> f64 {
  let helper = recur_fn(|helper, (a, product_state)| {
    if a > b {
      product_state
    } else {
      helper((next(a), term(a) * product_state))
    }
  });

  helper.call((a, 1.0))
}

pub fn factorial_by_product(n: i128) -> f64 {
  product(|i| i as f64, 1, |i| i + 1, n)
}

fn pi_again(n: i128) -> f64 {
  fn term(i: i128) -> f64 {
    let i2 = (i * 2) as f64;
    i2 * (i2 + 2.0) / ((i2 + 1.0) * (i2 + 1.0))
  }
  4.0 * product(term, 1, |i| i + 1, n / 2)
}

// Exercise 1.32
// generalize sum and product
pub fn accumulate(
  combiner: &impl Fn(f64, f64) -> f64,
  null_value: f64,
  term: impl Fn(i128) -> f64,
  a: i128,
  next: impl Fn(i128) -> i128,
  b: i128,
) -> f64 {
  if a > b {
    null_value
  } else {
    combiner(
      term(a),
      accumulate(combiner, null_value, term, next(a), next, b),
    )
  }
}

pub fn accumulate_iter(
  combiner: &impl Fn(f64, f64) -> f64,
  null_value: f64,
  term: impl Fn(i128) -> f64,
  a: i128,
  next: impl Fn(i128) -> i128,
  b: i128,
) -> f64 {
  let helper = recur_fn(|helper, (a, acc_state)| {
    if a > b {
      acc_state
    } else {
      helper((next(a), combiner(term(a), acc_state)))
    }
  });

  helper.call((a, null_value))
}

// Exercise 1.33
// generalize accululate
pub fn filtered_accumulate(
  combiner: &impl Fn(f64, f64) -> f64,
  null_value: f64,
  predicate: impl Fn(i128) -> bool,
  term: impl Fn(i128) -> f64,
  a: i128,
  next: impl Fn(i128) -> i128,
  b: i128,
) -> f64 {
  if a > b {
    null_value
  } else {
    if predicate(a) {
      combiner(
        term(a),
        filtered_accumulate(combiner, null_value, predicate, term, next(a), next, b),
      )
    } else {
      filtered_accumulate(combiner, null_value, predicate, term, next(a), next, b)
    }
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

fn intergral_simpson_rule(f: impl Fn(f64) -> f64, a: f64, b: f64, n: i128) -> f64 {
  let h = (b - a) / n as f64;
  let y = |k: i128| f(a + k as f64 * h);

  let term = |k| -> f64 {
    if k == 0 {
      f(a as f64)
    } else {
      if k % 2 == 0 {
        2.0 * y(k)
      } else {
        4.0 * y(k)
      }
    }
  };
  h / 3.0 * sum(term, 0, |i| i + 1, n)
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
    "sum cube of 1..10 with sum_iter {}",
    sum_iter(&cube_fn, 1, inc, 10)
  );
  println!("sum of cube of 1..10 {}", sum(cube_fn, 1, inc, 10));
  println!(
    "sum of square of prime of 1..10 {}",
    filtered_accumulate(
      &|a, b| a + b,
      0.0,
      is_prime,
      |i| (i * i) as f64,
      1,
      |i| i + 1,
      10
    )
  );
  println!(
    "product of all positive integers 1..11 that are relatively prime to 11 {}",
    filtered_accumulate(
      &|a, b| a * b,
      1.0,
      |i| gcd(i, 11) == 1,
      |i| i as f64,
      1,
      |i| i + 1,
      11
    )
  );
  println!(
    "sum of cube of 1..10 using iter {}",
    sum_iter(&cube_fn, 1, inc, 10)
  );
  println!("sum of 1..10 {}", sum_integers(1, 10));
  println!("pi simulation {}", 8.0 * pi_sum(1, 1000));
  println!(
    "pi simulation {}",
    8.0 * sum(|i| 1.0 / (i * (i + 2)) as f64, 1, |i| i + 4, 1000)
  );
  println!(
    "pi simulation using sum_iter {}",
    8.0 * sum_iter(&|i| 1.0 / (i * (i + 2)) as f64, 1, |i| i + 4, 1000)
  );
  println!(
    "pi simulation with accumulate {}",
    8.0
      * accumulate(
        &|a, b| a + b,
        0.0,
        |i| 1.0 / (i * (i + 2)) as f64,
        1,
        |i| i + 4,
        1000
      )
  );
  println!(
    "pi simulation with accumulate_iter {}",
    8.0
      * accumulate_iter(
        &|a, b| a + b,
        0.0,
        |i| 1.0 / (i * (i + 2)) as f64,
        1,
        |i| i + 4,
        1000
      )
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
  println!(
    "integral of cube with simpson's rule from 0 to 1 {}",
    intergral_simpson_rule(cube, 0.0, 1.0, 100)
  );
  println!(
    "factorial of 5 by product {}",
    product(|i| i as f64, 1, |i| i + 1, 5)
  );
  println!(
    "factorial of 5 by product_iter {}",
    product_iter(|i| i as f64, 1, |i| i + 1, 5)
  );
  println!("approximation of pi/4 {}", pi_again(4000));
}
