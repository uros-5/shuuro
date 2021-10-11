use crate::{
    board::{add_pieces, get, get_possible_range, piece_exist},
    piece_directions::{get_directions, get_directions_length},
    piece_enums::{Color, PieceName, PiecePins, ServerPiece, TypeOfSearch},
};
use std::collections::HashMap;

pub struct MoveGenerator {
    my_pos: i32,
    board: HashMap<i32, ServerPiece>,
    possible_range: Vec<i32>,
    checks: Vec<Vec<i32>>,
    pin: PiecePins,
    enemy_moves: Vec<i32>,
}

impl MoveGenerator {
    pub fn new(my_pos: i32) -> MoveGenerator {
        MoveGenerator {
            my_pos,
            board: add_pieces(),
            possible_range: get_possible_range(),
            checks: Vec::<Vec<i32>>::new(),
            pin: PiecePins::new(),
            enemy_moves: Vec::<i32>::new(),
        }
    }

    pub fn run(&mut self) -> Vec<i32> {
        let (my_piece, exist) = piece_exist(self.my_pos, &self.board);
        if exist {
            let king = self.get_my_king(&my_piece.color);
            let enemy_color = self.opposite_color(&my_piece.color);
            let mut my_moves = self.generate_moves(self.my_pos, &TypeOfSearch::MyMoves);
            self.enemy_moves(&enemy_color);
            self.find_pins(king);
            let is_check = self.is_check();
            self.checks.retain(|x| !x.is_empty());
            if self.my_pos == king {
                self.fix_check_by_king(is_check, &mut my_moves.0);
                return my_moves.0;
            }
            if self.pin.start == true {
                self.fixed_pin(is_check, &mut my_moves.0);
            } else if is_check == true {
                self.fix_check_by_piece(&mut my_moves.0);
            }
            my_moves.0
        } else {
            vec![]
        }
    }

    fn enemy_moves(&mut self, enemy_color: &Color) {
        for (pos, piece) in &self.board {
            if &piece.color == enemy_color {
                let mut moves = self.generate_moves(*pos, &TypeOfSearch::Check);
                self.enemy_moves.append(&mut moves.0);
                self.checks.push(moves.1);
            }
        }
    }

    // helper functions

    fn generate_moves(&self, pos: i32, type_of_search: &TypeOfSearch) -> (Vec<i32>, Vec<i32>) {
        fn clear(dir_moves: &mut Vec<i32>, final_moves: &mut Vec<i32>) {
            final_moves.append(dir_moves);
            dir_moves.clear();
        }

        let mut final_moves = Vec::<i32>::new();
        let mut checks = Vec::<i32>::new();
        let piece = get(pos, &self.board);
        let directions = get_directions(&piece.piece);
        let length = get_directions_length(&type_of_search, &piece.piece);
        let mut new_position = pos;
        let mut dir_moves: Vec<i32> = Vec::new();
        for direction in directions {
            if dir_moves.len() > 0 {
                clear(&mut dir_moves, &mut final_moves);
            }
            new_position = pos;
            loop {
                new_position += direction;
                if !self.possible_range.contains(&new_position) {
                    break;
                }
                let other_piece = get(new_position, &self.board);
                match other_piece.piece {
                    PieceName::NoPiece => dir_moves.push(new_position),
                    PieceName::Plynth => {
                        if &piece.piece == &PieceName::Night {
                            dir_moves.push(new_position);
                            break;
                        }
                    }
                    _ => {
                        if other_piece.color == piece.color {
                            break;
                        } else {
                            if type_of_search == &TypeOfSearch::Check {
                                if other_piece.piece == PieceName::King {
                                    dir_moves.push(new_position);
                                    &checks.push(pos);
                                    &checks.append(&mut dir_moves);
                                    //final_moves.append(&mut dir_moves);
                                    //dir_moves.clear();
                                    //break;
                                    continue;
                                }
                            }
                            dir_moves.push(new_position);
                            clear(&mut dir_moves, &mut final_moves);
                            break;
                        }
                    }
                }
                if length == true {
                    continue;
                } else {
                    if dir_moves.len() > 0 {
                        final_moves.append(&mut dir_moves);
                        dir_moves.clear();
                    }
                    break;
                }
            }
        }
        clear(&mut dir_moves, &mut final_moves);
        (final_moves, checks)
    }

    fn find_pins(&mut self, pos: i32) {
        let piece = get(pos, &self.board);
        let directions = get_directions(&piece.piece);
        let mut new_position = pos;
        let mut dir_moves: Vec<i32> = Vec::new();
        for direction in directions {
            new_position = pos;
            if !dir_moves.is_empty() {
                dir_moves.clear();
            }
            loop {
                new_position += direction;
                if !self.possible_range.contains(&new_position) {
                    break;
                }

                let other_piece = get(new_position, &self.board);
                match other_piece.piece {
                    PieceName::NoPiece => dir_moves.push(new_position),
                    PieceName::Plynth => {
                        break;
                    }
                    _ => {
                        if other_piece.color == piece.color {
                            if new_position == self.my_pos && self.pin.start == false {
                                self.pin.start = true;
                                dir_moves.push(new_position);
                                continue;
                            }
                            break;
                        } else {
                            if self.pin.start == true {
                                let dirs_check =
                                    get_directions(&other_piece.piece).contains(&direction);
                                let enemy_length = get_directions_length(
                                    &TypeOfSearch::MyMoves,
                                    &other_piece.piece,
                                );
                                if dirs_check && enemy_length == true {
                                    dir_moves.push(new_position);
                                    self.pin.fix = dir_moves.to_vec();
                                    return ();
                                }
                                self.pin.reset();
                            }
                            dir_moves.clear();
                            break;
                        }
                    }
                }
            }
        }
    }

    fn get_my_king(&self, color: &Color) -> i32 {
        for (pos, piece) in &self.board {
            if &piece.color == color && piece.piece == PieceName::King {
                return *pos;
            }
        }
        -1
    }

    fn is_check(&self) -> bool {
        for i in &self.checks {
            if !i.is_empty() {
                return true;
            }
        }
        return false;
    }

    fn fix_check_by_king(&self, check: bool, moves: &mut Vec<i32>) {
        if check {
            for checks in &self.checks {
                moves.retain(|x| !checks.contains(x));
                if moves.is_empty() {
                    break;
                }
            }
        }
        moves.retain(|x| !self.enemy_moves.contains(x));
    }

    fn fix_check_by_piece(&self, moves: &mut Vec<i32>) {
        for checks in &self.checks {
            moves.retain(|x| checks.contains(x));
            if moves.is_empty() {
                break;
            }
        }
    }

    fn fixed_pin(&self, is_check: bool, my_moves: &mut Vec<i32>) {
        my_moves.retain(|x| self.pin.fix.contains(x));
        if is_check == true {
            self.fix_check_by_piece(my_moves);
        }
    }

    fn opposite_color(&self, color: &Color) -> Color {
        match color {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
            Color::NoColor => Color::NoColor,
        }
    }
}
