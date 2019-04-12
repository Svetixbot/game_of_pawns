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
`place(piece, board): board`- just takes a piece and a board. and does the job; gives updated board back.

## How to test `place`:
1. assert that a piece is on the board after executing the function
2. assert that pieces are not placed on top of each other
3. assert that pawns are not on 1st and 8th rows
4. assert that KING is not close to the king

## `Place` should
1.
- find all the cells which are blocked for this piece:
  - all the busy cells, which is just a board represetation.
  - not allowed:
     - if a piece is a pawn - 1st and 8th are always blocked
     - if a piece is a king - KING's position + surrounding square is not allowed.

```
find(Q, board): blockedPositions.
```

2.
 - generate an available position for the board, which are not blocked.
```
  generate(blockedPositions): availablePosition
```

3.
 - update board and return it


 it would help to represent a board like this:
board = [{piece, (x,y)}] - array of placed pieces.
