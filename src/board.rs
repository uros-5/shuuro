pub fn b_range() -> Vec<i32> {
    let mut start = 0;
    let mut end = 12;
    let mut all_pos = Vec::<i32>::new();
    for _i in 0..12 {
        for i2 in start..end {
            all_pos.push(i2)
        }
        start += 16;
        end += 16;
    }
    all_pos
}

#[derive(PartialEq, Debug)]
pub enum Piece {
    King,
    Pawn,
    Bishop,
    Night,
    Rook,
    Queen,
    Plynth,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Color {
    Red,
    Blue,
    NoColor,
}

#[derive(Debug)]
pub struct ServerPiece {
    pub name: Piece,
    pub color: Color,
    pub pos: i32,
}

impl ServerPiece {
    fn new(name: Piece, color: Color, pos: i32) -> ServerPiece {
        ServerPiece { name, color, pos }
    }
}

pub struct Pin {
    pub start: bool,
    pub fix: Vec<i32>,
}

impl Pin {
    pub fn new() -> Pin {
        Pin {
            start: false,
            fix: Vec::<i32>::new(),
        }
    }

    pub fn reset(&mut self) {
        self.start = false;
        self.fix.clear();
    }
}

#[derive(PartialEq)]
pub enum Searching {
    Regular,
    Check,
    Pin,
}

pub struct Board {
    board: Vec<ServerPiece>,
    range: Vec<i32>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![],
            range: b_range(),
        }
    }

    pub fn add_data(&mut self) {
        self.board
            .push(ServerPiece::new(Piece::King, Color::Red, 154));
        self.board
            .push(ServerPiece::new(Piece::Queen, Color::Red, 105));
        self.board
            .push(ServerPiece::new(Piece::King, Color::Blue, 57));
        self.board
            .push(ServerPiece::new(Piece::Queen, Color::Blue, 54));
        self.board
            .push(ServerPiece::new(Piece::Rook, Color::Red, 48));
    }

    pub fn get(&self, pos: i32) -> Option<&ServerPiece> {
        self.board.iter().find(|x| x.pos == pos).map(|v| v)
    }

    pub fn get_all_pieces<'a, 'b>(&self, color: &'b Color) -> Vec<&ServerPiece> {
        self.board
            .iter()
            .filter(move |&x| &x.color == color)
            .collect()
    }

    pub fn get_king(&self, color: &Color) -> Option<&ServerPiece> {
        self.board
            .iter()
            .find(|x| x.name == Piece::King && &x.color == color)
    }

    pub fn get_enemy_color(&self, color: &Color) -> Color {
        match color {
            Color::Red => return Color::Blue,
            Color::Blue => return Color::Red,
            Color::NoColor => return Color::NoColor,
        };
    }

    pub fn in_range(&self, pos: i32) -> bool {
        if self.range.contains(&pos) {
            return true;
        } else {
            return false;
        }
    }
}
