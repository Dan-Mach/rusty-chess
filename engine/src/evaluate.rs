use crate::board::Board;
use crate::color::Color;
use crate::pieces::Piece;

pub fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0,
    }
}

const PAWN_PST: [i32; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
     50,  50,  50,  50,  50,  50,  50,  50,
     10,  10,  20,  30,  30,  20,  10,  10,
      5,   5,  10,  25,  25,  10,   5,   5,
      0,   0,   0,  20,  20,   0,   0,   0,
      5,  -5, -10,   0,   0, -10,  -5,   5,
      5,  10,  10, -20, -20,  10,  10,   5,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const KNIGHT_PST: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_PST: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

const ROOK_PST: [i32; 64] = [
      0,   0,   5,  10,  10,   5,   0,   0,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
      5,  10,  10,  10,  10,  10,  10,   5,
      0,   0,   5,  10,  10,   5,   0,   0,
];

const QUEEN_PST: [i32; 64] = [
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   0,   0,   0,   5,   0, -10,
    -10,   0,   5,   5,   5,   5,   5, -10,
     -5,   0,   5,   5,   5,   5,   0,  -5,
      0,   0,   5,   5,   5,   5,   0,  -5,
    -10,   5,   5,   5,   5,   5,   0, -10,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

const KING_PST: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -10, -20, -20, -20, -20, -20, -20, -10,
     20,  20,   0,   0,   0,   0,  20,  20,
     20,  30,  10,   0,   0,  10,  30,  20,
];

fn mirror_sq(sq: usize) -> usize {
    sq ^ 56
}

fn piece_square_value(piece: Piece, color: Color, sq: usize) -> i32 {
    let index = match color {
        Color::White => sq,
        Color::Black => mirror_sq(sq),
    };

    match piece {
        Piece::Pawn => PAWN_PST[index],
        Piece::Knight => KNIGHT_PST[index],
        Piece::Bishop => BISHOP_PST[index],
        Piece::Rook => ROOK_PST[index],
        Piece::Queen => QUEEN_PST[index],
        Piece::King => KING_PST[index],
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;
    let mut white_bishops = 0;
    let mut black_bishops = 0;

    for rank in 0..8 {
        for file in 0..8 {
            if let Some(piece) = board.squares[rank][file] {
                let sq = rank * 8 + file;
                let value = piece_value(piece.kind) + piece_square_value(piece.kind, piece.color, sq);

                match piece.color {
                    Color::White => {
                        score += value;
                        if piece.kind == Piece::Bishop {
                            white_bishops += 1;
                        }
                    }
                    Color::Black => {
                        score -= value;
                        if piece.kind == Piece::Bishop {
                            black_bishops += 1;
                        }
                    }
                }
            }
        }
    }

    if white_bishops >= 2 {
        score += 30;
    }
    if black_bishops >= 2 {
        score -= 30;
    }

    let side_to_move_mobility = board.generate_legal_moves().len() as i32;
    match board.active_color {
        Color::White => score += side_to_move_mobility * 2,
        Color::Black => score -= side_to_move_mobility * 2,
    }

    score
}