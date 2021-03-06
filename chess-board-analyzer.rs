/*
Find the winner (W or B) for the given chess board. If there isn't a King in checkmate position output N.

You have to make several assumptions:
- The given boards are legal and are assuming the official Chess rules: https://en.wikipedia.org/wiki/Rules_of_chess
- In every board there is a winner (no draws) or the board is not terminal (the game could be continued)
- An attacked King could be saved only by moving himself to a safe square (not by using another piece from the King's team)
- White pawns are moving upwards, while black pawns are moving downwards

Example board:

........
.......k
........
........
........
......R.
.K.....R
........


In this example the white rooks (uppercase R letters) are attacking all the squares the black king (lowercase k letter) could move onto,
 so the black king is in checkmate position and the white (W) player wins.
*/

use std::{io, fmt};
use std::iter::{FromIterator, Map};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::borrow::Borrow;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Board {
    base_layer: Vec<char>,
    // black_white: HashMap<bool, Vec<Vec<bool>>>,
    white_allowed: Vec<Vec<bool>>,
    black_allowed: Vec<Vec<bool>>,
}

#[derive(Clone, Copy)]
struct Offset {
    col: i8,
    row: i8,
}

impl Offset {
    // will return a vec containing this moves on rotation 0°, 90°, 180° and 270°
    // Probably not the most rust idiomatic way to do it ?
    fn all_axis(offsets: &Vec<Offset>) -> Vec<Vec<Offset>> {
        let axis1 = offsets.clone();
        let axis2: Vec<Offset> = offsets.iter().map(|offset| Offset {
            col: -offset.row,
            row: offset.col,
        }).collect();
        let axis3: Vec<Offset> = offsets.iter().map(|offset| Offset {
            col: -offset.col,
            row: -offset.row,
        }).collect();
        let axis4: Vec<Offset> = offsets.iter().map(|offset| Offset {
            col: offset.row,
            row: -offset.col,
        }).collect();
        vec![axis1, axis2, axis3, axis4]
    }
    fn rotate_90_cw(offsets: &Vec<Offset>) -> Vec<Offset> {
        offsets.iter().map(|offset| Offset {
            col: -offset.row,
            row: offset.col,
        }).collect()
    }
}


struct Piece {
    white: bool,
    c: char,
    allowed_moves: Vec<Vec<Offset>>,
}
impl Piece {
    fn new(white: bool, c: char, allowed_moves: Vec<Vec<Offset>>) -> Piece {
        let piece = Piece {
            white,
            c: if white { c.to_uppercase().next().unwrap() } else { c.to_lowercase().next().unwrap() },
            allowed_moves,
        };
        piece
    }
    fn just_inverse_color(to_inverse: &Piece) -> Piece {
        Piece::new(!to_inverse.white, to_inverse.c, to_inverse.allowed_moves.clone())
    }
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.white, self.c)
    }
}

fn main() {
    // A few key moves
    let up: Vec<Offset> = (1..8).map(|i| Offset { col: 0, row: i }).collect();
    let diag_up_right: Vec<Offset> = (1..8).map(|i| Offset { col: i, row: i }).collect();
    let l: Vec<Offset> = vec![Offset { col: 1, row: -2 }];
    let rev_l: Vec<Offset> = vec![Offset { col: -1, row: -2 }];
    // ignoring the special first move rule. It's a final board. Fingers crossed hoping it doesn't pop up. Ignoring also diag mvt rule has it's not relevant here
    let black_pawn = Piece::new(false, 'p', vec![vec![Offset { col: 0, row: -1 }], vec![Offset { col: -1, row: -1 }], vec![Offset { col: 1, row: -1 }]]);
    let white_pawn = Piece::new(true, 'p', vec![vec![Offset { col: 0, row: 1 }], vec![Offset { col: -1, row: 1 }], vec![Offset { col: 1, row: 1 }]]);
    let white_king = Piece::new(true, 'k', vec![vec![Offset { col: 0, row: -1 }], vec![Offset { col: -1, row: -1 }], vec![Offset { col: 1, row: -1 }], vec![Offset { col: 0, row: 1 }], vec![Offset { col: -1, row: 1 }], vec![Offset { col: 1, row: 1 }]]);
    let white_rook = Piece::new(true, 'r', Offset::all_axis(&up));
    let white_bishop = Piece::new(true, 'b', Offset::all_axis(&diag_up_right));
    let white_queen = Piece::new(true, 'Q', [white_bishop.allowed_moves.clone(), white_rook.allowed_moves.clone()].concat());
    let white_knight = Piece::new(true, 'N', [Offset::all_axis(&l), Offset::all_axis(&rev_l)].concat());
    let black_rook = Piece::just_inverse_color(&white_rook);
    let black_king = Piece::just_inverse_color(&white_king);
    let black_bishop = Piece::just_inverse_color(&white_bishop);
    let black_queen = Piece::just_inverse_color(&white_queen);
    let black_knight = Piece::just_inverse_color(&white_knight);
    let empty = Piece::new(true, '.', vec![]);
    // that's ugly but I got tired of fighting the borrow checker to do what I really it wanted. It won, for now
    let mut pieces: HashMap<char, &Piece> = HashMap::new();
    pieces.insert(black_pawn.c, &black_pawn);
    pieces.insert(white_pawn.c, &white_pawn);
    pieces.insert(white_king.c, &white_king);
    pieces.insert(white_rook.c, &white_rook);
    pieces.insert(white_bishop.c, &white_bishop);
    pieces.insert(white_queen.c, &white_queen);
    pieces.insert(white_knight.c, &white_knight);
    pieces.insert(black_rook.c, &black_rook);
    pieces.insert(black_king.c, &black_king);
    pieces.insert(black_bishop.c, &black_bishop);
    pieces.insert(black_queen.c, &black_queen);
    pieces.insert(black_knight.c, &black_knight);
    pieces.insert(empty.c, &empty);


    let mut base_layer = Vec::new();
    for i in 0..8 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let board_row = input_line.trim_matches('\n').to_string().chars().collect::<Vec<_>>();
        base_layer.push(board_row);
    }
    let mut board = Board {
        base_layer: base_layer.concat(),
        black_allowed: vec![vec![true; 8]; 8],
        white_allowed: vec![vec![true; 8]; 8],
    };
    let mut white_pos = Offset { row: 0, col: 0};
    let mut black_pos = Offset { row: 0, col: 0};

    // that indentation level is absolutely horrendous
    for (i, c) in board.base_layer.iter().enumerate() {
        let row: i8 = (i / 8) as i8;
        let col: i8 = (i % 8) as i8;
        let piece = pieces.get(&c).unwrap();
        if piece.c == white_king.c {
            white_pos.row = row;
            white_pos.col = col;
        } else if piece.c == black_king.c {
            black_pos.row = row;
            black_pos.col = col;
        } else if piece.c != '.' { // mark the pos of you own piece
            board.black_allowed[row as usize][col as usize] = false;
            board.white_allowed[row as usize][col as usize] = false;
        }
        for offsets in &piece.allowed_moves {
            for offset in offsets {
                let col_threat = (col + offset.col) as usize;
                let row_threat = (row + offset.row) as usize;
                if within_bounds(col_threat) && within_bounds(row_threat) {
                    let c = &board.base_layer[row_threat * 8 + col_threat];
                    let piece_there = pieces[&c];
                    let mut layer_to_mark = if piece.white { &mut board.black_allowed } else { &mut board.white_allowed };
                    if piece_there.c == empty.c {
                        layer_to_mark[row_threat][col_threat] = false;
                    } else {
                        if piece_there.white != piece.white {
                            layer_to_mark[row_threat][col_threat] = false;
                        }
                        if piece_there.c != white_king.c && piece_there.c != black_king.c {
                            break;
                        }
                    }
                }
            }
        }
    }


    let can_white_be_saved = can_be_saved(white_king, white_pos, &mut board.white_allowed, &mut board.base_layer);
    let can_black_be_saved = can_be_saved(black_king, black_pos, &mut board.black_allowed, &mut board.base_layer);

    eprintln!("BASE");
    for x in board.base_layer {
        eprintln!("{:?}", x);
    }
    eprintln!("WHITE ALLOWED");
    for row in board.white_allowed {
        for col in row {
            eprint!("{}", if col { "o " } else { "X " });
        }
        eprint!("\n");
    }
    eprintln!("BLACK ALLOWED");
    for row in board.black_allowed {
        for col in row {
            eprint!("{}", if col { "o " } else { "X " });
        }
        eprint!("\n");
    }
    eprintln!("white: {}  black: {}", can_white_be_saved, can_black_be_saved);

    println!("{}", if (can_white_be_saved && can_black_be_saved) || (!can_white_be_saved && !can_black_be_saved)  {
        'N'
    } else if can_black_be_saved {
        'B'
    } else {
        'W'
    });
}

fn within_bounds(coord: usize) -> bool {
    coord >= 0 && coord < 8
}

fn can_be_saved(king: Piece, mut king_pos: Offset, mut layer: &mut Vec<Vec<bool>>, mut base_layer: &mut Vec<char>) -> bool {
    let not_threaten = layer[king_pos.row as usize][king_pos.col as usize];
    let mut king_moves = king.allowed_moves
        .iter()
        .flatten();
    let mut adjacents = king_moves
        .filter(|offset| {
            within_bounds((king_pos.row + offset.row) as usize) && within_bounds((king_pos.col + offset.col) as usize)
        });
    let adjacent_free = adjacents.any(|offset| {
        layer[(king_pos.row + offset.row) as usize][(king_pos.col + offset.col) as usize]
    });
    let adjacent_with_enemy = adjacents.filter(|offset| {
        !layer[(king_pos.row + offset.row) as usize][(king_pos.col + offset.col) as usize]
    });

    not_threaten || (adjacent_free)
}
