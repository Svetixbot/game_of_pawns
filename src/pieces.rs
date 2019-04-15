
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::*;

pub fn generate_set() -> Vec<char> {
  let pieces: Vec<char> = "rnbqbnrppppppppRNBQBNRPPPPPPPP".chars().collect();
  let kings: Vec<char> = "Kk".chars().collect();
  let mut random_pieces = thread_rng();
  let mut random_size = thread_rng();
  let picked_pieces: Vec<char> = pieces.choose_multiple(&mut random_pieces, random_size.gen_range(0, 30)).cloned().collect();
  [&picked_pieces[..], &kings[..]].concat()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_always_has_kings() {
    assert!(generate_set().contains(&'K'));
    assert!(generate_set().contains(&'k'));
  }

  #[test]
  fn it_generates_different_set_everytime() {
    assert_ne!(generate_set(), generate_set())
  }
}