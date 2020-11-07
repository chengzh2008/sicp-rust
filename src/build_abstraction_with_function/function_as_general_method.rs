use crate::build_abstraction_with_function::high_order_functions::{cube, double};
use recur_fn::{recur_fn, RecurFn};

// lambda expression in rust is similar to closure
fn test_closure(n: i128) -> i128 {
  let helper = recur_fn(|helper, (i, acc)| if i > n { acc } else { helper((i + 1, i + acc)) });
  helper.call((1, 0))
}

fn average(i: f64, j: f64) -> f64 {
  (i + j) / 2.0
}

fn close_enough(a: f64, b: f64) -> bool {
  let delta = 0.00001;
  f64::abs(a - b) < delta
}

fn positive(i: f64) -> bool {
  i > 0.0
}

fn negative(i: f64) -> bool {
  i < 0.0
}

// finding roots of equations by the half-interval method
fn search(f: impl Fn(f64) -> f64, neg_point: f64, pos_point: f64) -> f64 {
  let mid_point = average(neg_point, pos_point);
  if close_enough(neg_point, pos_point) {
    mid_point
  } else {
    let test_value = f(mid_point);
    if negative(test_value) {
      search(f, mid_point, pos_point)
    } else if positive(test_value) {
      search(f, neg_point, mid_point)
    } else {
      mid_point
    }
  }
}

fn half_interval_search(f: impl Fn(f64) -> f64, neg_point: f64, pos_point: f64) -> f64 {
  let l = f(neg_point);
  let r = f(pos_point);
  if negative(l) && positive(r) {
    search(f, neg_point, pos_point)
  } else if positive(l) && negative(r) {
    search(f, pos_point, neg_point)
  } else {
    panic!("starting points are not usful");
  }
}

// fixpoint
fn fix_point(f: &impl Fn(f64) -> f64, guess: f64, track: bool) -> f64 {
  let helper = recur_fn(|helper, (i, guess)| {
    let r = f(guess);
    if track {
      println!("step: {} guess: {}", i, r);
    }
    if close_enough(r, guess) {
      r
    } else {
      helper((i + 1, r))
    }
  });
  helper.call((1, guess))
}

// square root by fixpoint
fn sqrt_by_fixpoint(x: f64) -> f64 {
  // however this won't converge as next guess y2 = x / y1, next guess y3 = x / y2 = y1
  // fix_point(&|y| x / y, 1.0)
  fix_point(&|y| average(y, x / y), 1.0, false)
}

#[test]
fn function_as_general_method_tests() {
  println!("apply closure with argument {}", (|i| i + 1)(3));
  println!("closure curry {}", (|i| { move |j| i + j })(3)(5));
  println!("closure recursion {}", test_closure(10));
  println!("closure recursion {}", f64::sin(3.1415));
  // approximate π as the root between 2 and 4 of sinx=0
  println!(
    "approximate pi {}",
    half_interval_search(f64::sin, 2.0, 4.0)
  );
  // search for a root of the equation x3−2x−3=0 between 1 and 2
  println!(
    "root of equation x3-2x-3=0 is {}",
    half_interval_search(|x| cube(x) - double(x) - 3.0, 1.0, 2.0)
  );
  // fixpoint of math.cos
  println!("fixpoint of cosine {}", fix_point(&f64::cos, 1.0, false));
  // find a solution to the equation y=siny+cosy
  println!(
    "solution for the equation {}",
    fix_point(&|y| f64::sin(y) + f64::cos(y), 1.0, false)
  );
  // sqrt by fixpoint
  println!("sqrt by fixpoint {}", sqrt_by_fixpoint(2.0));
  // golden ratio ϕ
  println!("golden ratio {}", fix_point(&|x| 1.0 + 1.0 / x, 1.0, false));
  // solve x ** x = 1000
  // stack overflow as it converges slower than the one with average damping
  println!(
    "solution for x ** x = 1000 {}",
    fix_point(&|x| f64::log10(1000.0) / f64::log10(x), 1.1, true)
  );
  println!(
    "solution for x ** x = 1000 with average damping {}",
    fix_point(
      &|x| average(x, f64::log10(1000.0) / f64::log10(x)),
      1.1,
      true
    )
  )
}
