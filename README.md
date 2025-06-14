# shuuro

A library for implementing Shuuro application.

[![crates.io](https://img.shields.io/crates/v/shuuro.svg)](https://crates.io/crates/shuuro)
[![docs.rs](https://docs.rs/shuuro/badge.svg)](https://docs.rs/shuuro/latest/shuuro/)

## Features

- Shuuro selection - selecting pieces:

```rust
use shuuro::{Shop, PieceType, Piece, Color, Move};
use shuuro::shuuro12::square12::Square12;
let mut shop = Shop::<Square12>::default();
for i in 0..5 {
    let piece = Piece{piece_type: PieceType::Queen, color: Color::Black};
    shop.play(Move::Select { piece });
}
assert_ne!(shop.to_sfen(Color::Blue), "kqqqq");
assert_eq!(shop.credit(Color::Blue), 800 - 110 * 3);
```

- Shuuro deploy - placing pieces on board:

```rust
use shuuro::{Position, PieceType, Color, consts::{D1, F12}, Piece, init};
use shuuro::shuuro12::{P12, Attack12};
Attacks12::init();
let mut pos = P12::default();
pos.set_hand("KQQNNBkrrrqnnPPP");
 
let white_king = (Piece{ piece_type: PieceType::King, color: Color::White }, D1);
let black_king = (Piece{ piece_type: PieceType::King, color: Color::Black }, F12);
pos.place(white_king.0, white_king.1 );
pos.place(black_king.0, black_king.1);
assert_eq!(pos.generate_sfen(), "3K8/12/12/12/12/12/12/12/12/12/12/5k6 r q3r2n2QB2N3P 1");
```

- Shuuro fight - play like normal chess:
```rust
use shuuro::*;
use shuuro::consts::*;
Attacks12::init();
let mut pos = P12::default();
pos.set_sfen("1K2RR6/PPP9/12/12/12/12/12/12/_.5_.5/pppppp6/1k64/12 r - 1");
let move_ = Move::Normal {from: B1, to: A1, promote: false};
pos.make_move(move_);
// Move can be made also with: pos.play("b1", "a1");
assert_eq!(pos.generate_sfen(), "K3RR6/PPP9/12/12/12/12/12/12/L05L05/pppppp6/1k10/12 b - 2");
```

Same API is available for standard chess(8x8).
