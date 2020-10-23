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


In this example the white rooks (uppercase R letters) are attacking all the squares the black king (lowercase k letter) could move onto, so the black king is in checkmate position and the white (W) player wins.
*/

use std::io;
use std::iter::FromIterator;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Board {
    base_layer: Vec<String>,
    white_allowed: Vec<String>,
    black_allowed: Vec<String>,
}

#[derive(Clone, Copy)]
struct Offset {
    x: i8,
    y: i8,
}

impl Offset {
    // will return a vec containing this moves on rotation 0°, 90°, 180° and 270°
    // Probably not the most rust idiomatic way to do it ?
    fn all_axis(offsets: &Vec<Offset>) -> Vec<Offset> {
        let axis1 = offsets.clone();
        let axis2: Vec<Offset> = offsets.iter().map(|offset| Offset {
            x: -offset.y,
            y: offset.x,
        }).collect();
        let axis3: Vec<Offset> = offsets.iter().map(|offset| Offset {
            x: -offset.x,
            y: -offset.y,
        }).collect();
        let axis4: Vec<Offset> = offsets.iter().map(|offset| Offset {
            x: offset.y,
            y: -offset.x,
        }).collect();
        [axis1, axis2, axis3, axis4].concat()
    }
    fn rotate_90_cw(offsets: &Vec<Offset>) -> Vec<Offset> {
        offsets.iter().map(|offset| Offset {
            x: -offset.y,
            y: offset.x,
        }).collect()
    }
}


struct Piece {
    white: bool,
    c: char,
    allowed_moves: Vec<Offset>,
}

impl Piece {
    fn new(white: bool, c: char, allowed_moves: Vec<Offset>) -> Piece {
        Piece {
            white,
            c: if white { c.to_uppercase().next().unwrap() } else { c.to_lowercase().next().unwrap() },
            allowed_moves,
        }
    }
    fn just_inverse_color(to_inverse: &Piece) -> Piece {
        Piece::new(!to_inverse.white, to_inverse.c, to_inverse.allowed_moves.clone())
    }
}


fn main() {
    // A few key moves
    let UP: Vec<Offset> = (1..8).map(|i| Offset { x: 0, y: i }).collect();
    let DIAG_UP_RIGHT: Vec<Offset> = (1..8).map(|i| Offset { x: i, y: i }).collect();
    let L: Vec<Offset> = vec![Offset { x: 1, y: -2 }];
    let REV_L: Vec<Offset> = vec![Offset { x: -1, y: -2 }];
    // ignoring the special first move rule. It's a final board. Fingers crossed hoping it doesn't pop up. Ignoring also diag mvt rule has it's not relevant here
    let BLACK_PAWN:   Piece = Piece::new(false, 'p', vec![Offset { x: 0, y: -1 }, Offset { x: -1, y: -1 }, Offset { x: 1, y: -1 }]);
    let WHITE_PAWN:   Piece = Piece::new(true,  'p', vec![Offset { x: 0, y: 1 },  Offset { x: -1, y: 1 },  Offset { x: 1, y: 1 }]);
    let WHITE_KING:   Piece = Piece::new(true,  'k', [WHITE_PAWN.allowed_moves, BLACK_PAWN.allowed_moves, ONE_LEFT, ONE_RIGHT, Offset { x: 1, y: 0 }, Offset { x: -1, y: 0 }].concat());
    let WHITE_ROOK:   Piece = Piece::new(true,  'r', Offset::all_axis(&UP));
    let WHITE_BISHOP: Piece = Piece::new(true,  'b', Offset::all_axis(&DIAG_UP_RIGHT));
    let WHITE_QUEEN:  Piece = Piece::new(true,  'Q', [WHITE_BISHOP.allowed_moves.clone(), WHITE_ROOK.allowed_moves.clone()].concat());
    let WHITE_KNIGHT: Piece = Piece::new(true,  'K', [Offset::all_axis(&L), Offset::all_axis(&REV_L)].concat());
    let BLACK_ROOK:   Piece = Piece::just_inverse_color(&WHITE_ROOK);
    let BLACK_KING:   Piece = Piece::just_inverse_color(&WHITE_KING);
    let BLACK_BISHOP: Piece = Piece::just_inverse_color(&WHITE_BISHOP);
    let BLACK_QUEEN:  Piece = Piece::just_inverse_color(&WHITE_QUEEN);
    let BLACK_KNIGHT: Piece = Piece::just_inverse_color(&WHITE_KNIGHT);
    let mut board = Vec::new();

    for i in 0..8 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let board_row = input_line.trim_matches('\n').to_string();
        board.push(board_row);
    }


    for row in board.iter() {
        eprintln!("{}", row);
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("boardStateChar");
}
