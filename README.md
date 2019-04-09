# Game of pawns

## Install rust and cargo
```
curl https://sh.rustup.rs -sSf | sh
```

## Run tests

```
cargo test
```

## To think about:
`place(piece, board)`- just takes a piece and a board. and does the job.

## How to test `place`:
1. assert that a piece is on the board after executing the function
2. assert that pieces are within the distance
3. assert that pieces are away from specific rows.