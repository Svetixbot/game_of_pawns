fn main() {
    println!("Hello, world!");
}

fn place(piece: char, board: &mut Vec<char>) -> &mut Vec<char> {
    board.push(piece);
    board
}

#[cfg(test)]
mod tests {
    use super::*;
    fn board() -> Vec<char> {
      (0..64).map(|_| '.').collect()
    }

    #[test]
    fn it_adds_piece_to_an_empty_board() {
      let initial_board = &mut board();
      let board = place('K', initial_board);
      assert!(board_has_all_pieces(*board, &mut vec!['K']));
    }

    #[test]
    fn it_doesnt_remove_already_placed_pieces() {
      let initial_board = &mut board();
      let board = place('K', initial_board);
      assert!(board_has_all_pieces(*board, &mut vec!['q', 'K']));
    }

    fn board_has_all_pieces(board: Vec<char>, pieces: &mut Vec<char>) -> bool {
      board.into_iter().filter(|cell| cell != &'.').collect::<Vec<char>>().sort() == pieces.sort()
    }
}