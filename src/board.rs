pub struct Board {
    pub squares: [[char;8];8]
}

impl Board {
    pub fn new() -> Self {
        let mut board = [[' '; 8];8];
        let piece_setup = [
            "rnbqkbnr",
            "pppppppp",
            " ",
            " ",
            " ",
            " ",
            "PPPPPPPPP",
            "RNBQKBNR",

        ];

        for (i, row) in piece_setup.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                board[i][j] = c;
            }
        }
        Self { squares:board}
    }

    pub fn print_board(&self) {
        println!(" a b c d e f g h");
        println!("--------------------");

        for (i, row) in self.squares.iter().enumerate() {
            print!("{} |", 8-i);

            for &square in row.iter() {
                print!("{}", square);
            }
            println!("|");
        }
        println!("-------------------");
    }

}