// use crate::board::*;

// // move evaluation
// use crate::pieces::Piece;
// use crate::color::Color;
// use crate::board::Board;

// fn piece_value(piece: Piece) -> i32 {
//     match piece {
//         Piece::Pawn => 100,
//         Piece::Knight => 320,
//         Piece::Bishop => 330,
//         Piece::Rook => 500,
//         Piece::Queen => 900,
//         Piece::King => 0, 
//     }
// }

// pub fn evaluate(board: &Board) -> i32 {
//     let mut score = 0;

//     for rank in 0..8 {
//         for file in 0..8{
//             if let Some(piece) = board.squares[rank][file] {
//                let value = piece_value(piece.kind);

//                if piece.color == Color::White {
//                    score += value;
//                } else {
//                    score -= value;
//                }
//             }
//         }
//     }
//     score
// }
// use crate::board::*;
// use crate::evaluate::*;

// pub fn minimax(board: &mut Board, depth: i32, maximizing: bool) -> i32 {
//     if depth == 0 {
//         return evaluate(board);
//     }

//     let moves = board.generate_legal_moves();

//     if moves.is_empty() {
//         return evaluate(board); // later: checkmate/stalemate handling
//     }

//     if maximizing {
//         let mut max_eval = i32::MIN;

//         for mv in moves {
//             let eval = minimax(board, depth - 1, false);
//             board.undo_move().unwrap();

//             max_eval = max_eval.max(eval);
//         }

//         max_eval
//     } else {
//         let mut min_eval = i32::MAX;

//         for mv in moves {
//             let eval = minimax(board, depth - 1, true);
//             board.undo_move().unwrap();

//             min_eval = min_eval.min(eval);
//         }

//         min_eval
//     }
// }

// pub fn alphabeta(
//     board: &mut Board,
//     depth: i32,
//     mut alpha: i32,
//     mut beta: i32,
//     maximizing: bool,
// ) -> i32 {
//     if depth == 0 {
//         return evaluate(board);
//     }

//     let moves = board.generate_legal_moves();

//     if maximizing {
//         let mut value = i32::MIN;

//         for mv in moves {
//             board.make_move(&mv);

//             let eval = alphabeta(board, depth - 1, alpha, beta, false);

//             board.undo_move().unwrap();

//             value = value.max(eval);
//             alpha = alpha.max(value);

//             if alpha >= beta {
//                 break;
//             }
//         }

//         value
//     } else {
//         let mut value = i32::MAX;

//         for mv in moves {
//             board.make_move(&mv);

//             let eval = alphabeta(board, depth - 1, alpha, beta, true);

//             board.undo_move().unwrap();

//             value = value.min(eval);
//             beta = beta.min(value);

//             if beta <= alpha {
//                 break;
//             }
//         }

//         value
//     }
// }

// #[test]
// fn test_evaluate() {
//     let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//     let board = Board::from_fen(fen).unwrap();

//     let score = evaluate(&board);
//     assert_eq!(score, 0);
// }
