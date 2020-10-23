fn factorial(n: i32) -> i32 {
  if n == 1 {
    1
  } else {
    n * factorial(n - 1)
  }
}

fn fact_iter(n: i32) -> i32 {
  fn helper(p: i32, c: i32, max_count: i32) -> i32 {
    if c > max_count {
      p
    } else {
      helper(p * c, c + 1, max_count)
    }
  }
  helper(1, 1, n)
}

fn inc(a: i32) -> i32 {
  a + 1
}

fn dec(a: i32) -> i32 {
  a - 1
}

fn plus(a: i32, b: i32) -> i32 {
  if a == 0 {
    b
  } else {
    plus(dec(a), inc(b))
  }
}

fn ackermann(a: i32, b: i32) -> i32 {
  if b == 0 {
    0
  } else {
    if a == 0 {
      2 * b
    } else {
      if b == 1 {
        2
      } else {
        ackermann(a - 1, ackermann(a, b - 1))
      }
    }
  }
}

fn f(n: i32) -> i32 {
  ackermann(0, n)
}

fn g(n: i32) -> i32 {
  ackermann(1, n)
}

fn h(n: i32) -> i32 {
  ackermann(2, n)
}

#[test]
fn functions_and_their_processes_tests() {
  println!("{}", factorial(5));
  println!("{}", fact_iter(5));
  println!("{}", plus(5, 13));
  println!("{}", ackermann(1, 10));
  println!("{}", ackermann(2, 4));
  println!("{}", ackermann(3, 3));
  println!("{}", f(3));
  println!("{}", g(3));
  println!("{}", h(3));
}
