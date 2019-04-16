# April Challenge: Game of Pawns

## Purpose

Generate a random chess position as an 8x8 grid or in FEN format. The position does not have to be realistic, but it must follow these rules:

    there is one and only one king of each color;
    the kings must not be placed on adjacent squares;
    there can not be any pawn in the promotion square (no white pawn in the eighth rank, and no black pawn in the first rank);
    including the kings, up to 32 pieces of either color can be placed. There is no requirement for material balance between sides, but the picking of pieces should comply with what is found in a regular chess set (e.g., 8 pawns/colour, 1 queen/colour, etc)
    (FEN requirement only) it is white's turn, both sides have lost castling rights and there is no possibility for en passant (the FEN should thus end in w - - 0 1).

Your code should be able to generate a reasonably representative sample of all possible positions.  For instance, solutions that always generate positions with the same number of pieces on the board, or with kings always on a corner, will be redirected to /dev/null.

Your solution should be runnable by a Go script that takes the type of output as a flag.  E.g.,

`./go.sh --grid // should produce the output as an 8x8 grid...`

```
 .  .  .  Q  .  .  b  .
 .  .  .  .  .  .  .  .
 R  .  .  b  .  .  r  .
 .  .  .  .  q  .  .  K
 .  .  .  .  P  .  .  .
 .  B  k  P  .  .  .  .
 .  n  .  .  .  .  .  .
 .  .  .  N  .  .  .  .
```

`./go.sh --fen // should produce the output as a FEN record...`

```
3Q2b1/8/R2b2r1/4q2K/4P3/1BkP4/1n6/3N4/ w - - 0 1
```

Note: with no flags, the solution should default to producing a FEN formatted result.  Any other type of input should generate an error.


## How to run

```
./go.sh --grid
```

```
./go.sh --fen
```

## Prerequisites

### Install rust and cargo
```
curl https://sh.rustup.rs -sSf | sh
```

## How to dev and tests

To run the tests:

```
cargo test
```

To run:

```
cargo run
```