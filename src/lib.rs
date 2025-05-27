// src/lib.rs (or main.rs if it's the crate root for these modules)

// ... your other mods ...
mod board;
pub use crate::board::*;

mod pieces;
pub use crate::pieces::*;

mod color;
pub use crate::color::*;
mod file;
pub use crate::file::*;

mod rank;
pub use crate::rank::*;

// Add the new move module
mod genmove;
pub use crate::genmove::*;

mod error;
pub use crate::error::Error;


mod bitboard;
pub use crate::bitboard::*;