use std::env;
mod printer;
mod board;
mod pieces;

fn main() {
  let mut board = board::empty_board();
  let pieces = pieces::generate_set();
  for piece in pieces {
    board::place(piece, &mut board);
  }

  let args: Vec<String> = env::args().collect();
  match &args.get(1).map(|v| &v[..]) {
    Some("--grid") => printer::print_8x8_grid(&board),
    _ => {
        printer::print_in_fen(&board)
    },
  }
}