use crate::build_abstraction_with_function::high_order_functions::{cube, double};
use recur_fn::{recur_fn, RecurFn};

// lambda expression in rust is similar to closure
fn test_closure(n: i128) -> i128 {
  let helper = recur_fn(|helper, (i, acc)| if i > n { acc } else { helper((i + 1, i + acc)) });
  helper.call((1, 0))
}

pub fn average(i: f64, j: f64) -> f64 {
  (i + j) / 2.0
}

pub fn close_enough(a: f64, b: f64) -> bool {
  let delta = 0.00001;
  f64::abs(a - b) < delta
}

pub fn square(i: f64) -> f64 {
  i * i
}

pub fn positive(i: f64) -> bool {
  i > 0.0
}

pub fn negative(i: f64) -> bool {
  i < 0.0
}

// Function as Returned Values
pub fn average_damp(f: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
  let help = move |x| average(x, f(x));
  help
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
pub fn fix_point(f: &impl Fn(f64) -> f64, guess: f64, track: bool) -> f64 {
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
  fix_point(&average_damp(|y| x / y), 1.0, false)
}

// cube root by fixpoint and average_damp
fn cube_root(x: f64) -> f64 {
  fix_point(&average_damp(|y| x / square(y)), 1.0, false)
}

// Exercise 1.37
fn cont_frac(n_term: &impl Fn(i128) -> f64, d_term: &impl Fn(i128) -> f64, k: i128) -> f64 {
  let helper = recur_fn(|helper, i| {
    if i > k {
      0.0
    } else {
      n_term(i) / (d_term(i) + helper(i + 1))
    }
  });
  helper.call(1)
}

fn cont_frac_iter(n_term: &impl Fn(i128) -> f64, d_term: &impl Fn(i128) -> f64, k: i128) -> f64 {
  let helper = recur_fn(|helper, (i, acc)| {
    if i == 0 {
      acc
    } else {
      helper((i - 1, n_term(i) / (d_term(i) + acc)))
    }
  });
  helper.call((k, 0.0))
}

// Exercise 1.39 Lambert's formula
fn tan_cf(x: f64, k: i128) -> f64 {
  let helper = recur_fn(|helper, i| {
    if i > k {
      0.0
    } else {
      let n = if i == 1 { x } else { x * x };
      let d = (double(i) - 1) as f64;
      n / (d - helper((i + 1)))
    }
  });
  helper.call((1))
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
  // cube root by fixpoint
  println!(
    "cube root by fixpoint with average_damp {}",
    cube_root(27.0)
  );
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
  );
  println!(
    "solution for x ** x = 1000 with average_damp hof {}",
    fix_point(
      &average_damp(|x| f64::log10(1000.0) / f64::log10(x)),
      1.1,
      true
    )
  );
  println!("approximate 1/ϕ {}", cont_frac(&|i| 1.0, &|i| 1.0, 1000));
  println!(
    "approximate 1/ϕ iterative {}",
    cont_frac_iter(&|i| 1.0, &|i| 1.0, 1000)
  );
  // approximate e
  println!(
    "approximate e iterative {}",
    2.0
      + cont_frac_iter(
        &|i| 1.0,
        &|i| {
          if i < 3 {
            i as f64
          } else {
            if i % 3 == 2 {
              (i - i / 3) as f64
            } else {
              1.0
            }
          }
        },
        20
      )
  );
  // Lamber's formula
  println!("approximate tan(x)  {}", tan_cf(3.0, 100));
  println!("tan(x)  {}", f64::tan(3.0));
}
