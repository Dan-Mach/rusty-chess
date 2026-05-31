# rusty-chess

Rusty-chess is a Rust chess engine library. It provides board representation, FEN parsing, legal move generation, game state evaluation, and search (minimax/alpha-beta). The repository is a Cargo workspace with a single crate: `engine`.

## Repository layout

- `engine/` — core chess engine crate (library + stub binary).
  - `src/board.rs` — board state, FEN parsing, move application/undo, game result checks.
  - `src/move_generation.rs`, `src/genmove.rs` — move generation and move types.
  - `src/evaluate.rs` — material + piece-square evaluation.
  - `src/search.rs` — minimax and alpha-beta search.
  - `src/tests/` — unit tests for move generation and game rules.
- `EXECUTIVE_SUMMARY.md`, `LOGIC_ERRORS_REPORT.md`, `README_LOGIC_ANALYSIS.md` — historical analysis and test coverage notes.
- `SECURITY.md` — security policy.
- `design.drawio` — design diagram (Draw.io).

## Requirements

- Rust toolchain (edition 2021). Install from https://rustup.rs/.

## Build

From the repository root:

```bash
cargo build
```

## Test

From the repository root:

```bash
cargo test
```

## Using the engine

`engine` is a library crate. You can use it from another Rust crate in this workspace or add a path dependency:

```toml
[dependencies]
engine = { path = "engine" }
```

Core APIs to look at:

- `Board::parse_fen(...)` / `Board::new()` — create a board.
- `Board::generate_legal_moves()` — list legal moves for the side to move.
- `Board::make_move(...)` / `Board::undo_move()` — apply and undo moves.
- `Board::update_game_result()` / `Board::is_checkmate()` / `Board::is_stalemate()` — game status.
- `search::minimax(...)` / `search::alphabeta(...)` — evaluate positions.
- `evaluate::evaluate(...)` — static position evaluation.

The `engine` binary entrypoint (`engine/src/main.rs`) is currently a stub, so there is no CLI or UCI interface yet. Use the library APIs directly or the tests as usage examples.

## Notes

The coordinate system uses array indices where rank 8 is index 0 and rank 1 is index 7 (matching FEN order). This is consistent across the engine, but keep it in mind when indexing the board.
