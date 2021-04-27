use crate::build_abstraction_with_function::function_as_general_method::average;
use crate::build_abstraction_with_function::functions_and_their_processes::gcd;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Debug)]
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
  // Exercise 2.1 handling sign of rational number
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

// Exercise 2.2
type Point = Pair<f64, f64>;
type Segment = Pair<Point, Point>;

fn make_segment(p1: Point, p2: Point) -> Segment {
  make_pair(p1, p2)
}

fn start_segment(s: Segment) -> Point {
  head(s)
}

fn end_segment(s: Segment) -> Point {
  tail(s)
}

fn make_point(x: f64, y: f64) -> Point {
  make_pair(x, y)
}

fn x_point(p: Point) -> f64 {
  head(p)
}

fn y_point(p: Point) -> f64 {
  head(p)
}

fn midpoint_segment(s: Segment) -> Point {
  let p1 = start_segment(s);
  let p2 = end_segment(s);
  make_point(
    average(x_point(p1), x_point(p2)),
    average(y_point(p1), y_point(p2)),
  )
}

// 2.1.3 What Is Meant by Data
// functional representation or message passing
#[derive(Debug)]
enum AorB<A, B> {
  Left(A),
  Right(B),
}

fn pair_fn<A, B>(a: A, b: B) -> impl FnOnce(bool) -> AorB<A, B> {
  let dispatch = |m| {
    if m {
      AorB::Left(a)
    } else {
      AorB::Right(b)
    }
  };
  dispatch
}

fn head_fn<A, B>(z: impl FnOnce(bool) -> AorB<A, B>) -> AorB<A, B> {
  z(true)
}

fn tail_fn<A, B>(z: impl FnOnce(bool) -> AorB<A, B>) -> AorB<A, B> {
  z(false)
}

// Exercise 2.4 an alternative functional representation
// TODO: type inference is not enough
/*
fn pair_fn2<A, B>(x: A, y: B) -> impl FnOnce(Fn(A, B) -> AorB<A, B>) -> AorB<A, B> {
  let helper = |m| m(x, y);
  helper
}

fn head_fn2<A, B>(z: impl FnOnce(Fn(A, B) -> AorB<A, B>) -> AorB<A, B>) -> A {
  let helper = |p, q| p;
  z(helper)
}
*/

#[test]
fn data_abstraction_tests() {
  println!(" rational number {}", make_rat(2, 5));
  println!(" rational number {}", make_rat(2, 6));
  println!(" rational number {}", make_rat(-2, 6));
  println!(" rational number {}", make_rat(-2, -6));
  println!(" rational number {}", make_rat(2, -6));
  println!(
    " midpoint of a segment {:?}",
    midpoint_segment(make_segment(make_point(1.0, 3.0), make_point(3.0, 5.0)))
  );
  // functional representation
  println!(
    " numerator of the rational number {:?}",
    head_fn(pair_fn(6, 4))
  );
  println!(
    " denominator of the rational number {:?}",
    tail_fn(pair_fn(6, 4))
  );
  println!(" either a or b {:?}", tail_fn(pair_fn("a", "b")));
}
