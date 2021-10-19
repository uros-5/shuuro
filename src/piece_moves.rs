use crate::{
    board::{Board, Color, Piece, Pin, Searching, ServerPiece},
    piece_directions::{get_directions, get_directions_length},
};

pub struct MoveGenerator<'a> {
    me: i32,
    board: &'a Board,
    my_moves: Vec<i32>,
    my_king: i32,
    pub enemy_moves: Vec<i32>,
    pub checks: Vec<Vec<i32>>,
    pins: Pin,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(me: i32, board: &Board) -> MoveGenerator {
        MoveGenerator {
            me,
            my_king: -1,
            board,
            my_moves: Vec::<i32>::new(),
            enemy_moves: Vec::<i32>::new(),
            checks: Vec::<Vec<i32>>::new(),
            pins: Pin::new(),
        }
    }

    pub fn run(&mut self) {
        let my_piece = self.board.get(self.me);
        match my_piece {
            Some(piece) => {
                let opposite_color = self.board.get_enemy_color(&piece.color);
                if opposite_color != Color::NoColor {
                    let king = self.board.get_king(&piece.color);
                    self.my_moves = self.find_legal_moves(self.me, Searching::Regular);
                    self.find_enemy_moves(&opposite_color);
                    self.find_pins(&king);
                    let is_check: bool = self.is_check();
                    if self.me == self.my_king {
                        self.fix_king_check(is_check);
                    } else if self.pins.start == true {
                        self.fix_pin(is_check);
                    } else if is_check {
                        self.fix_check();
                    }
                    println!("{:?}", self.my_moves);
                }
            }
            None => (),
        }
    }

    fn find_enemy_moves(&mut self, color: &Color) {
        let pieces: Vec<&ServerPiece> = self.board.get_all_pieces(&color);
        for piece in pieces {
            let mut moves = self.find_legal_moves(piece.pos, Searching::Check);
            self.enemy_moves.append(&mut moves);
        }
    }

    fn find_pins(&mut self, king: &Option<&ServerPiece>) {
        match king {
            Some(piece) => {
                if piece.pos != self.me {
                    self.my_king = piece.pos;
                    self.find_legal_moves(piece.pos, Searching::Pin);
                }
            }
            None => self.pins.reset(),
        }
    }

    fn is_check(&self) -> bool {
        let checks: Vec<&Vec<i32>> = self.checks.iter().filter(|x| x.len() > 0).collect();
        if checks.len() > 0 {
            return true;
        }
        return false;
    }

    fn fix_king_check(&mut self, is_check: bool) {
        if is_check {
            let checks: Vec<&Vec<i32>> = self.checks.iter().filter(|x| x.len() > 0).collect();
            for check in checks {
                self.my_moves.retain(|x| !check.contains(x));
                if self.my_moves.len() == 0 {
                    break;
                }
            }
        }
    }

    fn fix_pin(&mut self, is_check: bool) {
        self.my_moves.clear();
        self.my_moves = self.pins.fix.to_vec();
        if is_check {
            self.fix_check();
        }
    }

    fn fix_check(&mut self) {
        let checks: Vec<&Vec<i32>> = self.checks.iter().filter(|x| x.len() > 0).collect();
        for check in checks {
            self.my_moves.retain(|x| check.contains(x));
            if self.my_moves.len() == 0 {
                break;
            }
        }
    }

    pub fn find_legal_moves(&mut self, pos: i32, searching: Searching) -> Vec<i32> {
        let my = self.board.get(pos);
        match my {
            Some(my_piece) => {
                let directions = get_directions(&my_piece.name, &my_piece.color);
                let length = get_directions_length(&searching, &my_piece.name);
                let mut my_moves = Vec::<i32>::new();
                let mut dir_moves = Vec::<i32>::new();
                for direction in directions {
                    my_moves.append(&mut dir_moves);
                    let mut new_pos = pos;
                    self.pins.reset();
                    loop {
                        new_pos += direction;
                        // this square does not exist
                        if !self.board.in_range(new_pos) {
                            break;
                        } else {
                            let square = self.board.get(new_pos);
                            match square {
                                Some(other_piece) => {
                                    if other_piece.name == Piece::Plynth {
                                        // this is just for knight
                                        if my_piece.name == Piece::Night {
                                            dir_moves.push(new_pos);
                                            break;
                                        }
                                        break;
                                    } else if other_piece.color == my_piece.color {
                                        //this is me vs me
                                        match searching {
                                            Searching::Regular => break,
                                            Searching::Check => {
                                                dir_moves.push(new_pos);
                                                break;
                                            }
                                            Searching::Pin => {
                                                if new_pos == self.me {
                                                    self.pins.start = true;
                                                    continue;
                                                } else {
                                                    dir_moves.clear();
                                                    break;
                                                }
                                            }
                                        }
                                    } else {
                                        // this is enemy
                                        match searching {
                                            Searching::Regular => {
                                                dir_moves.push(new_pos);
                                                break;
                                            }
                                            Searching::Check => {
                                                if other_piece.name == Piece::King {
                                                    dir_moves.push(new_pos);
                                                    dir_moves.push(pos);
                                                    self.checks.push(dir_moves.to_vec());
                                                    dir_moves.clear();
                                                    continue;
                                                } else {
                                                    dir_moves.push(new_pos);
                                                    break;
                                                }
                                            }
                                            Searching::Pin => {
                                                if self.pins.start {
                                                    dir_moves.push(new_pos);
                                                    self.pins.fix = dir_moves.to_vec();
                                                    dir_moves.clear();
                                                    return vec![];
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }

                                None => dir_moves.push(new_pos),
                            }
                            if length == true {
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                }
                my_moves.append(&mut dir_moves);
                dir_moves.clear();
                my_moves
            }
            None => {
                vec![]
            }
        }
    }
}
