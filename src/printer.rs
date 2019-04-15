pub fn print_8x8_grid(board: &Vec<char>) {
  for row in 0..8 {
    let row = &board[8*row..8*row + 8];
    println!("{}", row.iter().map(|cell| cell).cloned().collect::<String>());
  }
}
