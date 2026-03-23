use crate::board::*;
use crate::evaluate::*;

pub fn minimax(board: &mut Board, depth: i32, maximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let moves = board.generate_legal_moves();

    if moves.is_empty() {
        return evaluate(board); // later: checkmate/stalemate handling
    }

    if maximizing {
        let mut max_eval = i32::MIN;

        for mv in moves {
            let eval = minimax(board, depth - 1, false);
            board.undo_move().unwrap();

            max_eval = max_eval.max(eval);
        }

        max_eval
    } else {
        let mut min_eval = i32::MAX;

        for mv in moves {
            let eval = minimax(board, depth - 1, true);
            board.undo_move().unwrap();

            min_eval = min_eval.min(eval);
        }

        min_eval
    }
}