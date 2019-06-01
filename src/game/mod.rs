pub struct Board {
    width: u8,
    height: u8,
    squares: Vec<Square>,
}

impl Board {
    pub fn new(width: u8, height: u8) -> Board {
        let mut squares: Vec<Square> = Vec::with_capacity(64);
        let square_count = width * height;
        for _ in 0..square_count {
            squares.push(Square::new())
        }
        Board {
            width: width,
            height: height,
            squares: squares,
        }
    }

    pub fn set_piece(&mut self, x: u8, y: u8, piece: PieceColor) {
        let i: usize = usize::from(y * self.width + x);
        self.squares[i] = Square {
            piece: Option::Some(piece),
        }
    }

    pub fn init_checkers(&mut self) {
        let white_squares = [
            (0, 1),
            (0, 3),
            (0, 5),
            (0, 7),
            (1, 0),
            (1, 2),
            (1, 4),
            (1, 6),
            (2, 1),
            (2, 3),
            (2, 5),
            (2, 7),
        ];
        let black_squares = [
            (5, 0),
            (5, 2),
            (5, 4),
            (5, 6),
            (6, 1),
            (6, 3),
            (6, 5),
            (6, 7),
            (7, 0),
            (7, 2),
            (7, 4),
            (7, 6),
        ];
        for coords in white_squares.iter() {
            self.set_piece(coords.1, coords.0, PieceColor::White);
        }
        for coords in black_squares.iter() {
            self.set_piece(coords.1, coords.0, PieceColor::Black);
        }
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..8 {
            for x in 0..8 {
                let index = usize::from(y * self.width + x);
                match self.squares[index].piece {
                    None => result.push('_'),
                    Some(PieceColor::Black) => result.push('B'),
                    Some(PieceColor::White) => result.push('W'),
                }
            }
            result.push('\n');
        }
        result
    }
}

pub struct Square {
    piece: Option<PieceColor>,
}

impl Square {
    pub fn new() -> Square {
        Square {
            piece: Option::None,
        }
    }

    pub fn is_occupied(&self) -> bool {
        match self.piece {
            None => false,
            _ => true,
        }
    }
}

pub enum PieceColor {
    Black,
    White,
}
