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

fn fac(n: i32) -> i32 {
  if n == 1 {
    1
  } else {
    n * fac(n - 1)
  }
}

fn fib(n: i32) -> i32 {
  if n < 2 {
    n
  } else {
    fib(n - 2) + fib(n - 1)
  }
}

// rust function can not access local variable
fn fib_iter(n: i32) -> i32 {
  fn helper(a: i32, b: i32, i: i32, n: i32) -> i32 {
    if i == n {
      b
    } else {
      helper(b, a + b, i + 1, n)
    }
  }
  helper(0, 1, 1, n)
}

/*
The number of ways to change amount a using n kinds of coins equals
- the number of ways to change amount a using all but the the first kind of coin, plus
- the number of ways to change amount (a - d) using all n kinds of coins where d is the value of the first kind of coin
*/

fn count_change(amount: i32) -> i32 {
  cc(amount, 6)
}

fn cc(amount: i32, coin_kind: i8) -> i32 {
  if amount == 0 {
    1
  } else {
    if amount < 0 || coin_kind == 0 {
      0
    } else {
      cc(amount, coin_kind - 1) + cc(amount - get_value(coin_kind), coin_kind)
    }
  }
}

fn get_value(coin_kind: i8) -> i32 {
  match coin_kind {
    6 => 100,
    5 => 50,
    4 => 25,
    3 => 10,
    2 => 5,
    1 => 1,
    _ => 0,
  }
}

/*
Exercise 1.11 A function f is defined by the rule that f(n)=n if n<3 and f(n)=f(n−1)+2f(n−2)+3f(n−3) if n≥3.
Write a JavaScript function that computes f by means of a recursive process. Write a function that computes f
by means of an iterative process.
*/
fn fn3(n: i32) -> i32 {
  if n < 3 {
    n
  } else {
    fn3(n - 1) + 2 * fn3(n - 2) + 3 * fn3(n - 3)
  }
}

fn fn3_iter(n: i32) -> i32 {
  fn helper(p3: i32, p2: i32, p1: i32, k: i32, n: i32) -> i32 {
    if k == n {
      p1
    } else {
      helper(p2, p1, 3 * p3 + 2 * p2 + p1, k + 1, n)
    }
  }
  return helper(0, 1, 2, 2, n);
}

// m >= n
fn pascal(m: i32, n: i32) -> i32 {
  if n == 0 || m == n {
    1
  } else {
    pascal(m - 1, n - 1) + pascal(m - 1, n)
  }
}

// pascal triangle with interative process
fn pascal_iter(m: usize, n: usize) -> i32 {
  fn helper(m: usize, n: usize, l: usize, pre_vec: Vec<i32>) -> i32 {
    if m == 0 || m == n {
      1
    } else {
      if l == m {
        pre_vec[n - 1] + pre_vec[n]
      } else {
        let mut new_vec = vec![];
        for (i, _) in pre_vec.iter().enumerate() {
          if i == 0 {
            new_vec.push(1);
          } else {
            new_vec.push(pre_vec[i - 1] + pre_vec[i])
          }
        }
        new_vec.push(1);
        helper(m, n, l + 1, new_vec.to_vec())
      }
    }
  }
  helper(m, n, 2, vec![1, 1])
}

fn cube(x: f32) -> f32 {
  x * x * x
}

fn p(x: f32) -> f32 {
  3.0 * x - 4.0 * cube(x)
}

fn sine(angle: f32) -> f32 {
  if f32::abs(angle) <= 0.1 {
    angle
  } else {
    p(sine(angle / 3.0))
  }
}

fn expt(b: i32, n: i32) -> i32 {
  if n == 0 {
    1
  } else {
    b * expt(b, n - 1)
  }
}

fn expt_iter(b: i32, n: i32) -> i32 {
  fn helper(c: i32, p: i32, b: i32, n: i32) -> i32 {
    if c == n {
      p
    } else {
      helper(c + 1, b * p, b, n)
    }
  }
  helper(0, 1, b, n)
}

fn is_even(n: i32) -> bool {
  n % 2 == 0
}

fn square(i: i32) -> i32 {
  i * i
}

fn half(i: i32) -> i32 {
  i / 2
}

fn fast_expt(b: i32, n: i32) -> i32 {
  if n == 1 {
    b
  } else {
    if is_even(n) {
      square(fast_expt(b, half(n)))
    } else {
      b * fast_expt(b, n - 1)
    }
  }
}

fn fast_expt_iter(b: i32, n: i32) -> i32 {
  fn helper(p: i32, b: i32, n: i32) -> i32 {
    if n == 0 {
      p
    } else {
      if is_even(n) {
        helper(p, square(b), half(n))
      } else {
        helper(b * p, b, n - 1)
      }
    }
  }
  helper(1, b, n)
}

fn double(x: i32) -> i32 {
  x * 2
}

fn times(a: i32, b: i32) -> i32 {
  if b == 0 {
    0
  } else {
    a + times(a, b - 1)
  }
}

fn times_iter(a: i32, b: i32) -> i32 {
  fn helper(s: i32, a: i32, b: i32) -> i32 {
    if b == 0 {
      s
    } else {
      if is_even(b) {
        helper(s, double(a), half(b))
      } else {
        helper(s + a, a, b - 1)
      }
    }
  }
  helper(0, a, b)
}

fn fast_fib(n: i32) -> i32 {
  fn helper(a: i32, b: i32, p: i32, q: i32, count: i32) -> i32 {
    if count == 0 {
      b
    } else {
      if is_even(count) {
        helper(
          a,
          b,
          square(p) + square(q),
          2 * p * q + square(q),
          half(count),
        )
      } else {
        helper(b * q + a * q + a * p, b * p + a * q, p, q, count - 1)
      }
    }
  }
  helper(1, 0, 0, 1, n)
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
  println!("{}", h(4));
  println!("{}", fac(5));
  println!("{}", fib(5));
  println!("{}", fib_iter(5));
  println!("{}", count_change(100));
  println!("{}", fn3(3));
  println!("{}", fn3(4));
  println!("{}", fn3_iter(3));
  println!("{}", fn3_iter(4));
  println!("{}", pascal(3, 2));
  println!("{}", pascal_iter(3, 2));
  println!("{}", pascal(4, 2));
  println!("{}", pascal_iter(4, 2));
  println!("{}", expt(4, 2));
  println!("{}", expt_iter(4, 2));
  println!("{}", fast_expt(4, 2));
  println!("{}", fast_expt_iter(4, 2));
  println!("{}", times(4, 2));
  println!("{}", times_iter(4, 2));
  println!("{}", fast_fib(5));
}
