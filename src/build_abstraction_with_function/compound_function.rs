pub fn hello() {
  println!("hello from compound_function");
}

fn square(x: i32) -> i32 {
  x * x
}

fn sum_of_squares(x: i32, y: i32) -> i32 {
  square(x) + square(y)
}

fn abs(x: i32) -> i32 {
  if x >= 0 {
    x
  } else {
    -x
  }
}

fn great_or_equal(x: i32, y: i32) -> bool {
  x > y || x == y
}

fn two_larges(x: i32, y: i32, z: i32) -> (i32, i32) {
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

fn sum_of_squares_of_two_larges(x: i32, y: i32, z: i32) -> i32 {
  let (a, b) = two_larges(x, y, z);
  sum_of_squares(a, b)
}

fn a_plus_abs_b(a: i32, b: i32) -> i32 {
  let plus = |a, b| a + b;
  let minus = |a, b| a - b;
  if b > 0 {
    plus(a, b)
  } else {
    minus(a, b)
  }
}

fn p() -> i32 {
  p()
}

// test suggest that rust is applicative-order evaluation
// arguments of function is evaluated first
fn test(x: i32, y: i32) -> i32 {
  if x == 0 {
    0
  } else {
    y
  }
}

#[test]
fn name() {
  assert_eq!(square(3), 9);
  assert_eq!(square(square(3)), 81);
  assert_eq!(sum_of_squares(3, 4), 25);
  assert_eq!(abs(-3), 3);
  assert!(great_or_equal(4, 4));
  assert!(great_or_equal(5, 4));
  assert_eq!(sum_of_squares_of_two_larges(1, 3, 4), 25);
  assert_eq!(a_plus_abs_b(3, 5), 8);
  assert_eq!(test(0, p()), 0); // failed due to stackoverflow
}
