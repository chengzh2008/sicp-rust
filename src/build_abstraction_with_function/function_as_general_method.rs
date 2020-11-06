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
  let delta = 0.001;
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
    search(f, l, r)
  } else if positive(l) && negative(r) {
    search(f, r, l)
  } else {
    panic!("starting points are not usful");
  }
}

#[test]
fn function_as_general_method_tests() {
  println!("apply closure with argument {}", (|i| i + 1)(3));
  println!("closure curry {}", (|i| { move |j| i + j })(3)(5));
  println!("closure recursion {}", test_closure(10));
  println!("closure recursion {}", f64::sin(3.1415));
  // approximate π as the root between 2 and 4 of sinx=0
  // TODO: why not working
  println!(
    "approximate pi {}",
    half_interval_search(f64::sin, 2.0, 4.0)
  );
}
