fn main() {
    println!("Hello, world!");
}

fn place(piece: char, offset: usize, board: &mut [char]) -> &mut [char] {
    board[offset] = piece;
    board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_piece_to_an_empty_board() {
        assert_eq!(place('K', 0, &mut ['.']), ['K']);
    }
}