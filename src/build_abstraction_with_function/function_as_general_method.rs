use recur_fn::{recur_fn, RecurFn};

// lambda expression in rust is similar to closure
fn test_closure(n: i128) -> i128 {
  let helper = recur_fn(|helper, (i, acc)| if i > n { acc } else { helper((i + 1, i + acc)) });
  helper.call((1, 0))
}

#[test]
fn function_as_general_method_tests() {
  println!("apply closure with argument {}", (|i| i + 1)(3));
  println!("closure curry {}", (|i| { move |j| i + j })(3)(5));
  println!("closure recursion {}", test_closure(10));
}
