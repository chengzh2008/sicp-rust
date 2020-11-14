use crate::build_abstraction_with_function::function_as_general_method::{
  average_damp, fix_point, square,
};
use crate::build_abstraction_with_function::high_order_functions::{cube, double};
use recur_fn::{recur_fn, RecurFn};

fn derivative(g: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
  let dx = 0.00001;
  let helper = move |x| (g(x + dx) - g(x)) / dx;
  helper
}

fn newton_transform(g: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
  let helper = move |x| x - g(x) / derivative(&g)(x);
  helper
}

fn newton_method(g: impl Fn(f64) -> f64, guess: f64) -> f64 {
  fix_point(&newton_transform(g), guess, false)
}

// more generic method
// TODO: need to figure out how to give the Type for transform
/*
fn fix_point_of_transform(g: impl Fn(f64) -> f64, transform: T?, guess: f64) -> f64 {
  fix_point(transform(g), guess, false)
}
*/

// Exercise 1.40
fn cubic(a: f64, b: f64, c: f64) -> impl Fn(f64) -> f64 {
  let helper = move |x| cube(x as f64) + a * square(x) + b * x + c;
  helper
}

fn double_apply(g: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
  let helper = move |x| g(g(x));
  helper
}

fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
  F: Fn(A) -> B,
  G: Fn(B) -> C,
{
  move |x| g(f(x))
}

/* run time error
fn nth_apply(g: impl Fn(f64) -> f64, n: i128) -> impl Fn(f64) -> f64 {
  let helper = move |x| {
    if n == 1 {
      g(x)
    } else {
      g(nth_apply(&g, n - 1)(x))
    }
  };
  helper
}
*/
// Exercise 1.43
fn nth_apply(g: impl Fn(f64) -> f64, n: i128, input: f64) -> f64 {
  let helper = recur_fn(|helper, (k, acc)| {
    println!("k {} acc {}", k, acc);
    if k == n {
      g(acc)
    } else {
      helper((k + 1, g(acc)))
    }
  });
  helper.call((1, input))
}

// Exercise 1.44
fn smooth(f: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
  let dx = 0.00001;
  let helper = move |x| (f(x - dx) + f(x) + f(x + dx)) / 3.0;
  helper
}

#[test]
fn newton_method_tests() {
  println!("derive of cube of 5 is {}", derivative(cube)(5.0));
  println!(
    "newton_method sqrt of 4 is {}",
    newton_method(|y| square(y) - 4.0, 1.0)
  );
  println!(
    "fix_point with average_damp sqrt of 4 is {}",
    fix_point(&average_damp(|y| 4.0 / y), 1.0, false)
  );
  // approximate zeros of cubic
  println!(
    "newton_method zeros of x**2 + 2x + 1 is {}",
    newton_method(cubic(0.0, 0.0, -27.0), 1.0)
  );
  println!(
    "double of double of double is {}",
    double_apply(double_apply(double))(1.0)
  );
  println!(
    "compose square and double is {}",
    compose(double, square)(1.0)
  );
  println!("repeated square 2 is {}", nth_apply(square, 2, 5.0));
  // this is not quite right
  println!(
    "repeated smooth cube is {}",
    nth_apply(smooth(cube), 5, 4.0)
  );
  // can not do that in rust
  println!(
    "repeated smooth cube is {}",
    smooth(smooth(smooth(smooth(smooth(cube)))))(4.0)
  );
}
