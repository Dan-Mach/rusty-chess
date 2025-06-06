use failure::Fail;


#[derive(Clone, Debug, Fail, PartialEq, Eq)]
pub enum FenParseError {
    #[fail(display = "Invalid FEN format structure: {}", _0)]
    InvalidFormat(String),
    #[fail(display = "Invalid piece character in FEN: '{}'", _0)]
    InvalidPiece(char),
    #[fail(display = "Invalid FEN rank (files do not sum to 8): '{}'", _0)]
    InvalidRankLength(String),
    #[fail(display = "Too many parts in FEN string")]
    TooManyParts,
    #[fail(display = "Not enough parts in FEN string (expected 6)")]
    NotEnoughParts,
    #[fail(display = "Invalid active color in FEN: {}", _0)]
    InvalidActiveColor(String),
    #[fail(display = "Invalid castling rights in FEN: {}", _0)]
    InvalidCastlingRights(String),
    #[fail(display = "Invalid en passant target square in FEN: {}", _0)]
    InvalidEnPassantTarget(String),
    #[fail(display = "Invalid halfmove clock in FEN: {}", _0)]
    InvalidHalfmoveClock(String),
    #[fail(display = "Invalid fullmove number in FEN: {}", _0)]
    InvalidFullmoveNumber(String),
}
#[derive(Clone, Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid FEN string provided: {}", fen_string)] // General FEN issue
    InvalidFenGeneral { fen_string: String },

    #[fail(display = "FEN parsing failed")] // Wraps the specific FenParseError
    FenParsing(#[cause] FenParseError),

    #[fail(display = "The board specified did not pass sanity checks. Are you sure the kings exist and the side to move cannot capture the opposing king?")]
    InvalidBoard,

    #[fail(display = "The string specified does not contain a valid algebraic notation square")]
    InvalidSquare,

    #[fail(display = "The string specified does not contain a valid SAN notation move")]
    InvalidSanMove,

    #[fail(display = "An attempt was made to create a move from an invalid UCI string")] // Corrected typo "atempt"
    InvalidUciMove,

    #[fail(display = "The string specified does not contain a valid rank")]
    InvalidRank,

    #[fail(display = "The string specified does not contain a valid file")]
    InvalidFile,
}