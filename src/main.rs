mod printer;
mod board;
mod pieces;

fn main() {
  let mut board = (0..64).map(|_| '.').collect();
  let pieces = pieces::generate_set();
  for piece in pieces {
    board::place(piece, &mut board);
  }
  printer::print_8x8_grid(&board);
}