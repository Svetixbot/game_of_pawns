
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::*;

const WHITE_KING: char = 'K';
const WHITE_QUEEN: char = 'Q';
const WHITE_BISHOP: char = 'B';
const WHITE_KNIGHT: char = 'N';
const WHITE_ROOK: char = 'R';
const WHITE_PAWN: char = 'P';

const BLACK_KING: char = 'k';
const BLACK_QUEEN: char = 'q';
const BLACK_BISHOP: char = 'b';
const BLACK_KNIGHT: char = 'n';
const BLACK_ROOK: char = 'r';
const BLACK_PAWN: char = 'p';

const KINGS: [char; 2] = [WHITE_KING, BLACK_KING];
const ALL_PIECES: [char; 30] =
  [
    WHITE_ROOK,
    WHITE_KNIGHT,
    WHITE_BISHOP,
    WHITE_QUEEN,
    WHITE_BISHOP,
    WHITE_KNIGHT ,
    WHITE_ROOK,
    WHITE_PAWN,
    WHITE_PAWN,
    WHITE_PAWN,
    WHITE_PAWN,
    WHITE_PAWN,
    WHITE_PAWN,
    WHITE_PAWN,
    WHITE_PAWN,
    BLACK_ROOK,
    BLACK_KNIGHT,
    BLACK_BISHOP,
    BLACK_QUEEN,
    BLACK_BISHOP,
    BLACK_KNIGHT ,
    BLACK_ROOK,
    BLACK_PAWN,
    BLACK_PAWN,
    BLACK_PAWN,
    BLACK_PAWN,
    BLACK_PAWN,
    BLACK_PAWN,
    BLACK_PAWN,
    BLACK_PAWN
  ];

pub fn generate_set() -> Vec<char> {
  let mut random_pieces = thread_rng();
  let mut random_size = thread_rng();
  let picked_pieces: Vec<char> =
    ALL_PIECES.choose_multiple(&mut random_pieces, random_size.gen_range(0, 30)).cloned().collect();
  [&picked_pieces[..], &KINGS[..]].concat()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_always_has_kings() {
    assert!(generate_set().contains(&WHITE_KING));
    assert!(generate_set().contains(&BLACK_KING));
  }

  #[test]
  fn it_generates_different_set_everytime() {
    assert_ne!(generate_set(), generate_set())
  }
}