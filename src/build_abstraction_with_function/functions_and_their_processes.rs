use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn factorial(n: i128) -> i128 {
  if n == 1 {
    1
  } else {
    n * factorial(n - 1)
  }
}

pub fn fact_iter(n: i128) -> i128 {
  fn helper(p: i128, c: i128, max_count: i128) -> i128 {
    if c > max_count {
      p
    } else {
      helper(p * c, c + 1, max_count)
    }
  }
  helper(1, 1, n)
}

pub fn inc(a: i128) -> i128 {
  a + 1
}

pub fn dec(a: i128) -> i128 {
  a - 1
}

pub fn plus(a: i128, b: i128) -> i128 {
  if a == 0 {
    b
  } else {
    plus(dec(a), inc(b))
  }
}

pub fn ackermann(a: i128, b: i128) -> i128 {
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

fn f(n: i128) -> i128 {
  ackermann(0, n)
}

fn g(n: i128) -> i128 {
  ackermann(1, n)
}

fn h(n: i128) -> i128 {
  ackermann(2, n)
}

pub fn fac(n: i128) -> i128 {
  if n == 1 {
    1
  } else {
    n * fac(n - 1)
  }
}

pub fn fib(n: i128) -> i128 {
  if n < 2 {
    n
  } else {
    fib(n - 2) + fib(n - 1)
  }
}

// rust function can not access local variable
pub fn fib_iter(n: i128) -> i128 {
  fn helper(a: i128, b: i128, i: i128, n: i128) -> i128 {
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

fn count_change(amount: i128) -> i128 {
  cc(amount, 6)
}

fn cc(amount: i128, coin_kind: i8) -> i128 {
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

fn get_value(coin_kind: i8) -> i128 {
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
fn fn3(n: i128) -> i128 {
  if n < 3 {
    n
  } else {
    fn3(n - 1) + 2 * fn3(n - 2) + 3 * fn3(n - 3)
  }
}

fn fn3_iter(n: i128) -> i128 {
  fn helper(p3: i128, p2: i128, p1: i128, k: i128, n: i128) -> i128 {
    if k == n {
      p1
    } else {
      helper(p2, p1, 3 * p3 + 2 * p2 + p1, k + 1, n)
    }
  }
  return helper(0, 1, 2, 2, n);
}

// m >= n
pub fn pascal(m: i128, n: i128) -> i128 {
  if n == 0 || m == n {
    1
  } else {
    pascal(m - 1, n - 1) + pascal(m - 1, n)
  }
}

// pascal triangle with interative process
pub fn pascal_iter(m: usize, n: usize) -> i128 {
  fn helper(m: usize, n: usize, l: usize, pre_vec: Vec<i128>) -> i128 {
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

pub fn cube(x: f32) -> f32 {
  x * x * x
}

fn p(x: f32) -> f32 {
  3.0 * x - 4.0 * cube(x)
}

pub fn sine(angle: f32) -> f32 {
  if f32::abs(angle) <= 0.1 {
    angle
  } else {
    p(sine(angle / 3.0))
  }
}

pub fn expt(b: i128, n: i128) -> i128 {
  if n == 0 {
    1
  } else {
    b * expt(b, n - 1)
  }
}

pub fn expt_iter(b: i128, n: i128) -> i128 {
  fn helper(c: i128, p: i128, b: i128, n: i128) -> i128 {
    if c == n {
      p
    } else {
      helper(c + 1, b * p, b, n)
    }
  }
  helper(0, 1, b, n)
}

pub fn is_even(n: i128) -> bool {
  n % 2 == 0
}

pub fn square(i: i128) -> i128 {
  i * i
}

pub fn half(i: i128) -> i128 {
  i / 2
}

pub fn fast_expt(b: i128, n: i128) -> i128 {
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

pub fn fast_expt_iter(b: i128, n: i128) -> i128 {
  fn helper(p: i128, b: i128, n: i128) -> i128 {
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

pub fn double(x: i128) -> i128 {
  x * 2
}

pub fn times(a: i128, b: i128) -> i128 {
  if b == 0 {
    0
  } else {
    a + times(a, b - 1)
  }
}

pub fn times_iter(a: i128, b: i128) -> i128 {
  fn helper(s: i128, a: i128, b: i128) -> i128 {
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

pub fn fast_fib(n: i128) -> i128 {
  fn helper(a: i128, b: i128, p: i128, q: i128, count: i128) -> i128 {
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

pub fn gcd(a: i128, b: i128) -> i128 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

pub fn devides(test_divisor: i128, n: i128) -> bool {
  n % test_divisor == 0
}

fn find_divisor(n: i128, test_divisor: i128) -> i128 {
  if square(test_divisor) > n {
    n
  } else {
    if devides(test_divisor, n) {
      test_divisor
    } else {
      find_divisor(n, test_divisor + 1)
    }
  }
}

pub fn smallest_divisor(n: i128) -> i128 {
  find_divisor(n, 2)
}

pub fn is_prime(n: i128) -> bool {
  smallest_divisor(n) == n
}

pub fn expmod(base: i128, exp: i128, m: i128) -> i128 {
  if exp == 0 {
    1
  } else {
    if is_even(exp) {
      // square after expmod, otherwise it will overflow easily
      square(expmod(base, half(exp), m)) % m
    } else {
      base * expmod(base, exp - 1, m) % m
    }
  }
}

// Fermat test
pub fn fermat_test(n: i128) -> bool {
  fn try_it(a: i128, n: i128) -> bool {
    expmod(a, n, n) == a
  }
  let mut rng = rand::thread_rng();
  try_it(rng.gen_range(2, n), n)
}

pub fn fast_is_prime(n: i128, times: i128) -> bool {
  if times == 0 {
    true
  } else {
    if fermat_test(n) {
      fast_is_prime(n, times - 1)
    } else {
      false
    }
  }
}

// Exercise 1.22
fn timed_prime_test(n: i128) -> bool {
  println!(" start testing: {}", n);
  let now = SystemTime::now();
  start_prime_test(n, now)
}

fn start_prime_test(n: i128, now: SystemTime) -> bool {
  if is_prime(n) {
    report_prime(now, n)
  } else {
    true
  }
}

fn report_prime(now: SystemTime, n: i128) -> bool {
  println!(" *** ");
  println!(" prime number is:  {}", n);
  println!("Time used: {}", get_lapsed_time_millis(now));
  /*
  match now.elapsed() {
    Ok(elapsed) => println!("Time used: {}", elapsed.as_millis()),
    Err(e) => println!("Error: {:?}", e),
  }
  */
  false
}

fn get_lapsed_time_millis(then: SystemTime) -> u128 {
  let new_now = SystemTime::now();
  new_now
    .duration_since(UNIX_EPOCH)
    .expect("Time")
    .as_millis()
    - then.duration_since(UNIX_EPOCH).expect("Time").as_millis()
}

// start is odd number
fn search_for_prime(start: i128, count: i128) {
  fn helper(start: i128, count: i128) {
    if count == 0 {
      return;
    } else {
      if timed_prime_test(start) {
        helper(start + 2, count)
      } else {
        helper(start + 2, count - 1)
      }
    }
  }
  helper(start, count)
}

// Exercise 1.27
fn test_carmichael_number(n: i128) {
  for i in 2..n {
    if expmod(i, n, n) == i {
      println!(" testing {}", i);
    }
  }
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
  println!("{}", gcd(2, 5));
  println!("{}", smallest_divisor(45));
  println!("{}", is_prime(5));
  println!("{}", fermat_test(5));
  println!("{}", timed_prime_test(16769023));
  search_for_prime(1111, 3);
  search_for_prime(11111, 3);
  search_for_prime(111111, 3);
  search_for_prime(1111111, 3);
  search_for_prime(11111111, 3);
  search_for_prime(111111111, 3);
  // carmichael number
  test_carmichael_number(2821);
}
