mod prime_number_trial;
use prime_number_trial::find_nth_prime_parallelized;
use std::time::Instant;

fn main() {
  let number = 20_000_000; // 1_000_000_000;
  let start = Instant::now();
  println!("{}", find_nth_prime_parallelized(number));
  println!("{:?}", start.elapsed());
}
