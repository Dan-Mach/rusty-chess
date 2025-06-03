use engine::{Board};
fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    match Board::parse_fen(fen) {
        Ok(board) => {
            println!("{}", board);
        }
        Err(e) => {
            eprintln!("Error parsing FEN: {}", e);
        }
    }
}