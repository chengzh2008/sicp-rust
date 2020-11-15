use crate::build_abstraction_with_function::functions_and_their_processes::gcd;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
struct Pair<A, B>(A, B);

impl<A, B> Display for Pair<A, B>
where
  A: Display,
  B: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{} / {}", self.0, self.1)
  }
}

type PairInt = Pair<i128, i128>;

// tools for the rational number implementation
fn make_pair<A, B>(a: A, b: B) -> Pair<A, B> {
  Pair(a, b)
}

fn head<A, B>(x: Pair<A, B>) -> A {
  x.0
}

fn tail<A, B>(x: Pair<A, B>) -> B {
  x.1
}

// implementation of a rational number
fn make_rat(n: i128, d: i128) -> PairInt {
  let g = gcd(n, d);
  if n * d < 0 {
    make_pair(-i128::abs(n / g), i128::abs(d / g))
  } else {
    make_pair(i128::abs(n / g), i128::abs(d / g))
  }
}

fn numer(x: PairInt) -> i128 {
  head(x)
}

fn denom(x: PairInt) -> i128 {
  tail(x)
}

fn add_rat(x: PairInt, y: PairInt) -> PairInt {
  make_rat(
    numer(x) * denom(y) + numer(y) * denom(x),
    denom(x) * denom(y),
  )
}

fn sub_rat(x: PairInt, y: PairInt) -> PairInt {
  make_rat(
    numer(x) * denom(y) - numer(y) * denom(x),
    denom(x) * denom(y),
  )
}

fn mul_rat(x: PairInt, y: PairInt) -> PairInt {
  make_rat(numer(x) * numer(y), denom(x) * denom(y))
}

fn dev_rat(x: PairInt, y: PairInt) -> PairInt {
  make_rat(numer(x) * denom(y), denom(x) * numer(y))
}

fn equal_rat(x: PairInt, y: PairInt) -> bool {
  numer(x) == numer(y) && denom(x) == denom(y)
}

#[test]
fn data_abstraction_tests() {
  println!(" rational number {}", make_rat(2, 5));
  println!(" rational number {}", make_rat(2, 6));
  println!(" rational number {}", make_rat(-2, 6));
  println!(" rational number {}", make_rat(-2, -6));
  println!(" rational number {}", make_rat(2, -6));
}
