pub fn hello() {
  println!("hello from compound_function");
}

fn square(x: f32) -> f32 {
  x * x
}

fn sum_of_squares(x: f32, y: f32) -> f32 {
  square(x) + square(y)
}

fn abs(x: f32) -> f32 {
  if x >= 0.0 {
    x
  } else {
    -x
  }
}

fn great_or_equal(x: f32, y: f32) -> bool {
  x > y || x == y
}

fn two_larges(x: f32, y: f32, z: f32) -> (f32, f32) {
  if great_or_equal(x, y) {
    if great_or_equal(y, z) {
      (x, y)
    } else {
      (x, z)
    }
  } else {
    if great_or_equal(z, x) {
      (y, z)
    } else {
      (x, y)
    }
  }
}

fn sum_of_squares_of_two_larges(x: f32, y: f32, z: f32) -> f32 {
  let (a, b) = two_larges(x, y, z);
  sum_of_squares(a, b)
}

fn a_plus_abs_b(a: f32, b: f32) -> f32 {
  let plus = |a, b| a + b;
  let minus = |a, b| a - b;
  if b > 0.0 {
    plus(a, b)
  } else {
    minus(a, b)
  }
}

fn p() -> f32 {
  p()
}

// test suggest that rust is applicative-order evaluation
// arguments of function is evaluated first
fn test(x: f32, y: f32) -> f32 {
  if x == 0.0 {
    0.0
  } else {
    y
  }
}

fn good_enough(guess: f32, x: f32) -> bool {
  let next = improve(guess, x / guess);
  relative_error(guess, next)
}

fn relative_error(a: f32, b: f32) -> bool {
  let delta = 0.0001;
  f32::abs(a - b) / b <= delta
}

fn average(a: f32, b: f32) -> f32 {
  (a + b) / 2.0
}

fn improve(a: f32, b: f32) -> f32 {
  average(a, b)
}

fn improve_cube(a: f32, b: f32) -> f32 {
  (b / a * a + 2.0 * a) / 3.0
}

fn sqrt_iter(guess: f32, x: f32) -> f32 {
  if good_enough(guess, x) {
    guess
  } else {
    sqrt_iter(improve(guess, x / guess), x)
  }
}

fn cube_iter(guess: f32, x: f32) -> f32 {
  if good_enough(guess, x) {
    guess
  } else {
    sqrt_iter(improve_cube(guess, x), x)
  }
}

fn sqrt(x: f32) -> f32 {
  sqrt_iter(1.0, x)
}

fn cube(x: f32) -> f32 {
  cube_iter(1.0, x)
}

#[test]
fn elements_of_programming_tests() {
  println!("{}", square(3.0));
  println!("{}", square(square(3.0)));
  println!("{}", sum_of_squares(3.0, 4.0));
  println!("{}", abs(-3.0));
  assert!(great_or_equal(4.0, 4.0));
  assert!(great_or_equal(5.0, 4.0));
  println!("{}", sum_of_squares_of_two_larges(1.0, 3.0, 4.0));
  println!("{}", a_plus_abs_b(3.0, 5.0));
  // println!("{}", test(0, p()), 0); // failed due to stackoverflow
  println!("{}", sqrt(4.0));
  println!("{}", cube(81.0));
}
