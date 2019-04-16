use rand::prelude::*;

const EMPTY: char = '.';
static PROMOTION_SQUARE: [usize; 16] = [0,1,2,3,4,5,6,7,56,57,58,59,60,61,62,63];

pub fn empty_board() -> Vec<char> {
  (0..64).map(|_| EMPTY).collect()
}

pub fn place(piece: char, board: &mut Vec<char>) {
  let already_blocked_cells: Vec<usize> = blocked_cells(piece, board.to_vec());
  let available_cell = next_available_cell(already_blocked_cells);
  board[available_cell] = piece;
}

fn blocked_cells(piece: char, board: Vec<char>) -> Vec<usize>  {
  let mut already_blocked_cells = Vec::new();
  let another_king_position = board.iter().position(|&r| r == 'k' || r == 'K');

  for cell_index in 0..board.len() {
    if board[cell_index] != EMPTY
      || pawn_in_the_promotion_square(piece, cell_index)
        || another_king_adjacent(piece, cell_index, another_king_position) {
      already_blocked_cells.push(cell_index)
    }
  }
  already_blocked_cells
}

fn pawn_in_the_promotion_square(piece: char, cell_index: usize) -> bool {
  (piece == 'p' || piece == 'P') && PROMOTION_SQUARE.contains(&cell_index)
}

fn another_king_adjacent(piece: char, cell_index: usize, maybe_another_king: Option<usize>) -> bool {
  match maybe_another_king {
    Some(king_position) => {
        if piece == 'k' || piece == 'K' {
          let left_position = if king_position < 10 { 0 } else { king_position - 10 };
          let blockedking_position: Vec<usize> = (left_position .. king_position + 10).collect();
          return blockedking_position.contains(&cell_index);
        }
        return false;
      }
    None => return false,
  }

}

fn next_available_cell(blocked_cells: Vec<usize>) -> usize {
  let mut rng = thread_rng();
  loop {
    let next_random_cell = rng.gen_range(0, 63);
    if !blocked_cells.contains(&next_random_cell) {
      return next_random_cell;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds_piece_to_an_empty_board() {
    let mut board = empty_board();
    place('K', &mut board);
    assert_eq!(only_pieces(board), vec!['K']);
  }

  #[test]
  fn it_doesnt_remove_already_placed_pieces() {
    let mut board = empty_board();
    let mut expected = vec!['q', 'K'];
    place('q', &mut board);
    place('K', &mut board);
    board.sort();
    expected.sort();
    assert_eq!(only_pieces(board), expected);
  }

  #[test]
  fn it_doesnt_place_pawns_to_promotion_square() {
    let mut board = empty_board();
    place('p', &mut board);
    let expected: Vec<char> = (0..8).map(|_| EMPTY).collect();
    assert_eq!(&board[0..8], expected.as_slice());
    assert_eq!(&board[56..64], expected.as_slice());
    assert_eq!(only_pieces(board), vec!['p']);
  }

  #[test]
  fn it_doesnt_place_kings_on_adjacent_squares() {
    let mut board = empty_board();
    place('k', &mut board);
    place('K', &mut board);
    assert!(kings_are_not_adjacent(board))
  }

  #[test]
  fn it_doesnt_repeat_the_placements() {
    let mut first_board = empty_board();
    place('k', &mut first_board);
    let first_piece_position = first_board.iter().position(|&r| r == 'k').unwrap();

    let mut second_board = empty_board();
    place('k', &mut second_board);
    let second_piece_position = second_board.iter().position(|&r| r == 'k').unwrap();

    assert_ne!(first_piece_position, second_piece_position)
  }

  fn only_pieces(board: Vec<char>) -> Vec<char> {
    board.into_iter().filter(|&cell| cell != EMPTY).collect()
  }

  fn kings_are_not_adjacent(board: Vec<char>) -> bool {
    let black_king_position = board.iter().position(|&r| r == 'k').unwrap();
    let white_king_position = board.iter().position(|&r| r == 'K').unwrap();
    white_king_position.wrapping_sub(black_king_position) >= 10
  }
}