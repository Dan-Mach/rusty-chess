
use crate::board::Board;
use crate::genmove::Move;
use crate::pieces::{Piece, ColoredPiece};
use crate::color::Color;
use crate::error::Error;
use crate::coordinates::{square_to_array_indices, array_indices_to_square};
use crate::rank::Rank;
use crate::file::File;
use std::fmt;
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::convert::TryFrom;

//not yet implemented


