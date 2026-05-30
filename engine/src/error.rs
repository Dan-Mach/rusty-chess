use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum FenParseError {
    #[error("Invalid FEN format structure: {0}")]
    InvalidFormat(String),
    #[error("Invalid piece character in FEN: '{0}'")]
    InvalidPiece(char),
    #[error("Invalid FEN rank (files do not sum to 8): '{0}'")]
    InvalidRankLength(String),
    #[error("Too many parts in FEN string")]
    TooManyParts,
    #[error("Not enough parts in FEN string (expected 6)")]
    NotEnoughParts,
    #[error("Invalid active color in FEN: {0}")]
    InvalidActiveColor(String),
    #[error("Invalid castling rights in FEN: {0}")]
    InvalidCastlingRights(String),
    #[error("Invalid en passant target square in FEN: {0}")]
    InvalidEnPassantTarget(String),
    #[error("Invalid halfmove clock in FEN: {0}")]
    InvalidHalfmoveClock(String),
    #[error("Invalid fullmove number in FEN: {0}")]
    InvalidFullmoveNumber(String),
}
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("Invalid FEN string provided: {}", fen_string)] // General FEN issue
    InvalidFenGeneral { fen_string: String },

    #[error("FEN parsing failed")] // Wraps the specific FenParseError
    FenParsing(#[source] FenParseError),        
    #[error("The board specified did not pass sanity checks. Are you sure the kings exist and the side to move cannot capture the opposing king?")]
    InvalidBoard,

    #[error("The string specified does not contain a valid algebraic notation square")]
    InvalidSquare,

    #[error("The string specified does not contain a valid SAN notation move")]
    InvalidSanMove,

    #[error("An attempt was made to create a move from an invalid UCI string")] // Corrected typo "atempt"
    InvalidUciMove,

    #[error("The string specified does not contain a valid rank")]
    InvalidRank,

    #[error("The string specified does not contain a valid file")]
    InvalidFile,
}