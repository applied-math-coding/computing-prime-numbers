use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

type Prime = u64;

/// Decides if n is a prime by using the known primes 'primes'.
/// n must not be larger than the square of the largest element in primes!
fn is_prime_mem(n: Prime, primes: &Vec<Prime>) -> bool {
  for p in primes.iter() {
    if p * p > n {
      break;
    }
    if n > 6 && (n + 1) % 6 != 0 && (n - 1) % 6 != 0 {
      return false;
    }
    if n % p == 0 {
      return false;
    }
  }
  return true;
}

pub fn find_nth_prime_parallelized(n: u32) -> Prime {
  let mut primes = Arc::new(vec![2, 3, 5, 7]);
  let mut start = primes[primes.len() - 1] + 2;
  let mut count = primes.len();
  loop {
    let loop_time = Instant::now();
    let (end, mut prime_map) = find_next_primes(Arc::clone(&primes), start);
    let mut keys = prime_map.keys().map(|e| *e).collect::<Vec<usize>>();
    keys.sort();
    let mut next_primes = Arc::try_unwrap(primes).unwrap();
    for i in keys.iter() {
      if let Some(v) = prime_map.get_mut(i) {
        if v.len() + count < n as usize {
          count += v.len();
          next_primes.append(v);
        } else if count + 1 <= n as usize {
          return v[(n as usize - count - 1) as usize];
        } else {
          panic!("something went wrong!");
        }
      }
    }
    primes = Arc::new(next_primes);
    start = end + 1;
    println!(
      "count: {}, prime: {}, took: {:?}",
      count,
      primes[primes.len() - 1],
      loop_time.elapsed()
    );
  }
}

/// Based on the known primes 'primes', this method tries to find as much as possible primes
/// up from start.
fn find_next_primes(
  primes: Arc<Vec<Prime>>,
  mut start: Prime,
) -> (Prime, HashMap<usize, Vec<Prime>>) {
  const INTERVAL: Prime = 10_000_000;
  let max_prime = primes[primes.len() - 1];
  let max_end = max_prime * max_prime;
  let mut end = start;
  let (tx, rx) = mpsc::channel::<(usize, Vec<Prime>)>();
  for i in 0..num_cpus::get() {
    if start >= max_end {
      break;
    }
    end = *vec![start + INTERVAL, max_end].iter().min().unwrap();
    let tx_clone = tx.clone();
    let primes_clone = Arc::clone(&primes);
    thread::spawn(move || {
      tx_clone
        .send((i, find_primes(start, end, &primes_clone)))
        .unwrap();
    });
    start = end + 1;
  }
  drop(tx);
  let mut res = HashMap::new();
  for (i, next_primes) in rx {
    res.insert(i, next_primes);
  }
  (end, res)
}

/// Using the known primes 'primes', this method returns all primes within start and end.
/// It is assumed end is not beyond p*p, where p is the maximal prime from primes.
fn find_primes(start: Prime, end: Prime, primes: &Vec<Prime>) -> Vec<Prime> {
  let mut p = if start % 2 == 0 { start + 1 } else { start };
  let mut res = vec![];
  while p <= end {
    if is_prime_mem(p, primes) {
      res.push(p);
    }
    p += 2;
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_nth_prime_parallelized() {
    assert_eq!(find_nth_prime_parallelized(9), 23);
  }
}
