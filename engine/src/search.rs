use crate::board::Board;
use crate::color::Color;
use crate::coordinates::square_to_array_indices;
use crate::evaluate::{evaluate, piece_value};
use crate::genmove::{Move, MoveList};use crate::board::*;


const CHECKMATE_SCORE: i32 = 100_000;
const STALEMATE_SCORE: i32 = 0;

fn move_order_score(board: &Board, mv: &Move) -> i32 {
    let (to_r, to_f) = square_to_array_indices(mv.to);
    let mut score = 0;

    if let Some(captured) = board.squares[to_r][to_f] {
        score += 10_000 + piece_value(captured.kind);
    }

    if let Some(promo) = mv.promotion {
        score += 20_000 + piece_value(promo);
    }

    score
}

fn ordered_moves(board: &Board) -> MoveList {
    let mut moves = board.generate_legal_moves();
    moves.sort_by_key(|mv| -move_order_score(board, mv));
    moves
}

fn terminal_score(board: &Board, depth: i32) -> Option<i32> {
    let moves = board.generate_legal_moves();
    if !moves.is_empty() {
        return None;
    }

    match board.game_result {
        GameResult::Checkmate(winner) => {
            if winner == Color::White {
                Some(CHECKMATE_SCORE - depth)
            } else {
                Some(-CHECKMATE_SCORE + depth)
            }
        }
        GameResult::Stalemate => Some(STALEMATE_SCORE),
        GameResult::InProgress => Some(STALEMATE_SCORE),
    }
}

pub fn minimax(board: &mut Board, depth: i32, maximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    if let Some(score) = terminal_score(board, depth) {
        return score;
    }

    let moves = ordered_moves(board);

    if maximizing {
        let mut max_eval = i32::MIN;

        for mv in moves {
            board.make_move(&mv);
            let eval = minimax(board, depth - 1, false);
            board.undo_move().unwrap();

            max_eval = max_eval.max(eval);
        }

        max_eval
    } else {
        let mut min_eval = i32::MAX;

        for mv in moves {
            board.make_move(&mv);
            let eval = minimax(board, depth - 1, true);
            board.undo_move().unwrap();

            min_eval = min_eval.min(eval);
        }

        min_eval
    }
}

fn quiescence(board: &mut Board, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
    let stand_pat = evaluate(board);

    if maximizing {
        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }
    } else {
        if stand_pat <= alpha {
            return alpha;
        }
        if stand_pat < beta {
            beta = stand_pat;
        }
    }

    let mut moves = board.generate_legal_moves();
    moves.retain(|mv| {
        let (to_r, to_f) = square_to_array_indices(mv.to);
        board.squares[to_r][to_f].is_some() || mv.promotion.is_some()
    });
    moves.sort_by_key(|mv| -move_order_score(board, mv));

    if maximizing {
        let mut value = stand_pat;

        for mv in moves {
            board.make_move(&mv);
            let score = quiescence(board, alpha, beta, false);
            board.undo_move().unwrap();

            value = value.max(score);
            alpha = alpha.max(value);

            if alpha >= beta {
                break;
            }
        }

        value
    } else {
        let mut value = stand_pat;

        for mv in moves {
            board.make_move(&mv);
            let score = quiescence(board, alpha, beta, true);
            board.undo_move().unwrap();

            value = value.min(score);
            beta = beta.min(value);

            if beta <= alpha {
                break;
            }
        }

        value
    }
}

pub fn alphabeta(
    board: &mut Board,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
) -> i32 {
    if let Some(score) = terminal_score(board, depth) {
        return score;
    }

    if depth == 0 {
        return quiescence(board, alpha, beta, maximizing);
    }

    let moves = ordered_moves(board);

    if maximizing {
        let mut value = i32::MIN;

        for mv in moves {
            board.make_move(&mv);
            let eval = alphabeta(board, depth - 1, alpha, beta, false);
            board.undo_move().unwrap();

            value = value.max(eval);
            alpha = alpha.max(value);

            if alpha >= beta {
                break;
            }
        }

        value
    } else {
        let mut value = i32::MAX;

        for mv in moves {
            board.make_move(&mv);
            let eval = alphabeta(board, depth - 1, alpha, beta, true);
            board.undo_move().unwrap();

            value = value.min(eval);
            beta = beta.min(value);

            if beta <= alpha {
                break;
            }
        }

        value
    }
}

pub fn find_best_move(board: &mut Board, depth: i32) -> Option<Move> {
    let moves = ordered_moves(board);
    if moves.is_empty() {
        return None;
    }

    let maximizing = board.active_color == Color::White;
    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    for mv in moves {
        board.make_move(&mv);
        let score = alphabeta(board, depth - 1, i32::MIN, i32::MAX, !maximizing);
        board.undo_move().unwrap();

        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        } else if score < best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    best_move
}