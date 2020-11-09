use crate::build_abstraction_with_function::function_as_general_method::{
  average_damp, fix_point, square,
};
use crate::build_abstraction_with_function::high_order_functions::cube;

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
fn fix_point_of_transform(g: impl Fn(f64) -> f64, transform: T, guess: f64) -> f64 {
  fix_point(transform(g), guess, false)
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
}
