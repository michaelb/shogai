use crate::invalidmoveerror::*;
use crate::movement::*;
use crate::piece::*;
use crate::position::*;

use arrayvec::ArrayVec;
use std::iter::once;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Rules {
    pub can_uncover_check: bool,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            can_uncover_check: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Board {
    white_pawns: ArrayVec<[Piece; 32]>,
    black_pawns: ArrayVec<[Piece; 32]>,
    white_pieces: ArrayVec<[Piece; 32]>,
    black_pieces: ArrayVec<[Piece; 32]>,
    pub turn: Color,
    pub rules: Rules,
}
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lines = ["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        let mut counter = 0;
        write!(f, " 9 8 7 6 5 4 3 2 1 \n")?;
        write!(f, "+------------------+\n")?;
        for line in 0..9 {
            write!(f, "|")?;
            for column in 0..9 {
                if let Some(p) = &self.is_occupied_by(Position(line * 9 + (8 - column))) {
                    write!(f, "{}", p)?;
                } else {
                    write!(f, "  ")?;
                }
            }
            write!(f, "|{}\n", lines[counter])?;
            counter += 1;
        }
        write!(f, "+------------------+\n")?;
        write!(f, "White reserve: ")?;
        for white_reserve in self
            .iter()
            .filter(|piece| piece.position == None && piece.color == Color::White)
        {
            write!(f, "{}, ", white_reserve)?;
        }
        write!(f, "\nBlack reserve: ")?;

        for white_reserve in self
            .iter()
            .filter(|piece| piece.position == None && piece.color == Color::Black)
        {
            write!(f, "{}, ", white_reserve)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn empty() -> Self {
        Board {
            white_pawns: ArrayVec::<[Piece; 32]>::new(),
            black_pawns: ArrayVec::<[Piece; 32]>::new(),
            white_pieces: ArrayVec::<[Piece; 32]>::new(),
            black_pieces: ArrayVec::<[Piece; 32]>::new(),
            turn: Color::White,
            rules: Rules::default(),
        }
    }
    pub fn get_color(&self) -> Color {
        return self.turn;
    }

    fn push(&mut self, piece: Piece) {
        if piece.piecetype == PieceType::Pawn {
            if piece.color == Color::White {
                self.white_pawns.push(piece);
            } else {
                self.black_pawns.push(piece);
            }
        } else {
            if piece.color == Color::White {
                self.white_pieces.push(piece);
            } else {
                self.black_pieces.push(piece);
            }
        }
    }
    fn remove(&mut self, piece: Piece) {
        if piece.piecetype == PieceType::Pawn {
            if piece.color == Color::White {
                if let Some(index) = self.white_pawns.iter().position(|&p| p == piece) {
                    self.white_pawns.remove(index);
                    return;
                }
            } else {
                if let Some(index) = self.black_pawns.iter().position(|&p| p == piece) {
                    self.black_pawns.remove(index);
                    return;
                }
            }
        } else {
            if piece.color == Color::White {
                if let Some(index) = self.white_pieces.iter().position(|&p| p == piece) {
                    self.white_pieces.remove(index);
                    return;
                }
            } else {
                if let Some(index) = self.black_pieces.iter().position(|&p| p == piece) {
                    self.black_pieces.remove(index);
                    return;
                }
            }
        }
    }

    ///replace a piece by another piece (usually the same piece moved to another position)
    /// rely on the predicate that the first piece exists
    fn replace(&mut self, piece: Piece, new_piece: Piece) {
        //optimize in case it's just a normal move
        if piece.color == new_piece.color && piece.piecetype == new_piece.piecetype {
            if piece.piecetype == PieceType::Pawn {
                if piece.color == Color::White {
                    if let Some(index) = self.white_pawns.iter().position(|&p| p == piece) {
                        self.white_pawns[index] = new_piece;
                        return;
                    }
                } else {
                    if let Some(index) = self.black_pawns.iter().position(|&p| p == piece) {
                        self.black_pawns[index] = new_piece;
                        return;
                    }
                }
            } else {
                if piece.color == Color::White {
                    if let Some(index) = self.white_pieces.iter().position(|&p| p == piece) {
                        self.white_pieces[index] = new_piece;
                        return;
                    }
                } else {
                    if let Some(index) = self.black_pieces.iter().position(|&p| p == piece) {
                        self.black_pieces[index] = new_piece;
                        return;
                    }
                }
            }
        } else {
            // pop, push
            self.remove(piece);
            self.push(new_piece);
        }
    }

    pub fn withdraw(&mut self, c: Color) {
        let pawns: ArrayVec<[Piece; 32]> = self
            .iter_pawns(c)
            .filter(|p| p.color == c)
            .map(|&p| p)
            .collect();
        let pieces: ArrayVec<[Piece; 32]> = self
            .iter_pieces(c)
            .filter(|p| p.color == c)
            .map(|&p| p)
            .collect();
        if c == Color::White {
            self.white_pawns = pawns;
            self.white_pieces = pieces;
        } else {
            self.black_pawns = pawns;
            self.black_pieces = pieces;
        }
    }

    pub fn value(&self) -> i32 {
        self.iter()
            .map(|piece| {
                if piece.color == self.turn {
                    piece.value()
                } else {
                    -piece.value()
                }
            })
            .sum()
    }

    ///says if we are checkmated
    pub fn game_over(&self) -> bool {
        let mut can_always_take_the_king = true;
        for my_possible_move in self.iter_moves_partial_check() {
            let board_before_next = self.play_move_unchecked(&my_possible_move);
            let mut have_move_that_take_the_king = false;
            for opponent_next_move in board_before_next.iter_normal_moves_only(false) {
                let board_after_next_move =
                    board_before_next.play_move_unchecked(&opponent_next_move);
                if !board_after_next_move.contains(PieceType::King, self.get_color()) {
                    have_move_that_take_the_king = true;
                }
            }
            can_always_take_the_king &= have_move_that_take_the_king;
        }
        return can_always_take_the_king;
    }

    pub fn contains(&self, pc: PieceType, color: Color) -> bool {
        if pc == PieceType::Pawn {
            for piece in self.iter_pawns(color) {
                if piece.color == color && piece.piecetype == pc {
                    return true;
                }
            }
        } else {
            for piece in self.iter_pieces(color) {
                if piece.color == color && piece.piecetype == pc {
                    return true;
                }
            }
        }
        return false;
    }

    ///capture a piece if there, do nothing else
    fn capture_piece(&mut self, p: Position) {
        let my_color = self.get_color();
        let mut opponent_color = my_color;
        let mut old_piece = Piece {
            piecetype: PieceType::Pawn,
            color: Color::White,
            position: Some(Position(99)),
            promoted: true,
        }; //impossible piece just case it ends up in replace(.,.)
        let mut new_piece = old_piece;
        let mut piece_was_captured = false;
        opponent_color.invert();
        for piece in self.iter_pawns(opponent_color) {
            if piece.color == opponent_color && piece.position == Some(p) {
                piece_was_captured = true;
                new_piece = *piece;
                old_piece = *piece;
                new_piece.color.invert();
                new_piece.position = None;
                new_piece.promoted = false;
            }
        }
        for piece in self.iter_pieces(opponent_color) {
            if piece.color == opponent_color && piece.position == Some(p) {
                piece_was_captured = true;
                new_piece = *piece;
                old_piece = *piece;
                new_piece.color.invert();
                new_piece.position = None;
                new_piece.promoted = false;
            }
        }
        if piece_was_captured {
            self.replace(old_piece, new_piece);
        }
    }

    pub fn play_move(&self, mv: &str) -> Board {
        return self.play_move_general(mv, true);
    }
    pub fn play_move_unchecked(&self, mv: &str) -> Board {
        return self.play_move_general(mv, false);
    }

    fn play_move_general(&self, mv: &str, check: bool) -> Board {
        let mut new_board = self.clone();
        if mv == "restart" {
            println!("restart");
            return Self::new();
        }

        if check {
            if let Err(e) = self.check_move(mv) {
                //move not valid
                panic!("Invalid movement : {}", e);
            }
        }
        let movement: Movement = mv.parse().unwrap();

        //movement was checked so it's ok to just play
        if movement.start != None {
            // the movement is a normal movement
            //
            //if a piece (an opponent's) is here at the destination, remove it, change its color,
            new_board.capture_piece(movement.end);

            //then move the piece
            let exact_piece = self.is_occupied_by(movement.start.unwrap()).unwrap();
            let mut new_piece = exact_piece;
            new_piece.position = Some(movement.end);
            new_piece.promoted |= movement.promotion;

            new_board.replace(exact_piece, new_piece);
        } else {
            // the movement is a drop
            let exact_piece = *new_board
                .iter()
                .find(|piece| {
                    piece.piecetype == movement.piecetype
                        && piece.position == None
                        && piece.color == self.get_color()
                })
                .unwrap();

            let mut new_piece = exact_piece;
            new_piece.position = Some(movement.end);
            new_piece.promoted = false;

            new_board.replace(exact_piece, new_piece);
        }
        new_board.turn.invert();
        new_board
    }

    pub fn check_move<'a>(&'a self, mv: &'a str) -> Result<&'a str, InvalidMoveError> {
        // checks should be performed in this order
        Ok(mv)
            .and_then(check_syntax)
            .and_then(check_in_board)
            .and_then(|mv| check_start(mv, self))
            .and_then(|mv| check_destination(mv, self))
            .and_then(|mv| check_possible_move(mv, self))
            .and_then(|mv| check_nifu(mv, self))
            .and_then(|mv| check_move_possible_after_drop(mv, self))
            .and_then(|mv| check_promotion(mv, self))
            .and_then(|mv| check_uncover_check(mv, self))
            .and_then(|mv| check_checkmate_by_pawn_drop(mv, self))
    }

    pub fn check_move_general<'a>(
        &'a self,
        mv: &'a str,
        complete_check: bool,
    ) -> Result<&'a str, InvalidMoveError> {
        // checks should be performed in this order
        if complete_check {
            return self.check_move(mv);
        } else {
            return Ok(mv)
                .and_then(check_syntax)
                .and_then(check_in_board)
                .and_then(|mv| check_start(mv, self))
                .and_then(|mv| check_destination(mv, self))
                .and_then(|mv| check_possible_move(mv, self))
                .and_then(|mv| check_nifu(mv, self))
                .and_then(|mv| check_move_possible_after_drop(mv, self))
                .and_then(|mv| check_promotion(mv, self));
        }
    }

    pub fn is_occupied_by(&self, pos: Position) -> Option<Piece> {
        for &p in self.iter() {
            if p.position == Some(pos) {
                return Some(p);
            }
        }
        None
    }

    pub fn add_piece(&mut self, piece: Piece) {
        if piece.piecetype == PieceType::Pawn {
            if piece.color == Color::White {
                self.white_pawns.push(piece);
            } else {
                self.black_pawns.push(piece);
            }
        } else {
            if piece.color == Color::White {
                self.white_pieces.push(piece);
            } else {
                self.black_pieces.push(piece);
            }
        }
    }

    ///return the list of the piece
    pub fn export(&self) -> Vec<Piece> {
        return self.iter().map(|&p| p).collect();
    }

    ///a central symmetry is needed
    pub fn flip(&mut self) {
        let mut tmp: Vec<(Piece, Piece)> = Vec::new();
        for piece in self.iter() {
            let pos = piece.position;
            if let Some(x) = pos {
                let i = x.0 % 9;
                let j = x.0 / 9;
                let new_x = (8 - j) * 9 + (8 - i);
                let new_piece = Piece {
                    color: piece.color,
                    piecetype: piece.piecetype,
                    promoted: piece.promoted,
                    position: Some(Position(new_x)),
                };
                let old_piece = *piece;

                tmp.push((new_piece, old_piece));
            }
        }

        for (new_piece, old_piece) in tmp.iter() {
            self.replace(*old_piece, *new_piece);
        }
    }

    ///set the regular starting position for one player
    pub fn set(&mut self, col: Color) {
        for i in 18..27 {
            let p = Piece {
                color: col,
                piecetype: PieceType::Pawn,
                promoted: false,
                position: Some(Position(i)),
            };
            self.add_piece(p);
        }
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Lance,
            promoted: false,
            position: Some(Position(0)),
        });
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Lance,
            promoted: false,
            position: Some(Position(8)),
        });

        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Knight,
            promoted: false,
            position: Some(Position(1)),
        });

        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Knight,
            promoted: false,
            position: Some(Position(7)),
        });

        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Silver,
            promoted: false,
            position: Some(Position(2)),
        });
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Silver,
            promoted: false,
            position: Some(Position(6)),
        });
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Gold,
            promoted: false,
            position: Some(Position(3)),
        });
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Gold,
            promoted: false,
            position: Some(Position(5)),
        });
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::King,
            promoted: false,
            position: Some(Position(4)),
        });
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Rook,
            promoted: false,
            position: Some(Position(16)),
        });
        self.add_piece(Piece {
            color: col,
            piecetype: PieceType::Bishop,
            promoted: false,
            position: Some(Position(10)),
        });
    }

    pub fn new<'a>() -> Board {
        let mut b = Board::empty();
        //white is always 'up' ( in the a-b-c rows)
        b.set(Color::Black);
        b.flip();
        b.set(Color::White);
        b
    }
    pub fn iter(&self) -> impl Iterator<Item = &Piece> {
        self.white_pawns
            .iter()
            .chain(self.black_pawns.iter())
            .chain(self.white_pieces.iter())
            .chain(self.black_pieces.iter())
    }
    pub fn iter_pawns(&self, c: Color) -> impl Iterator<Item = &Piece> {
        if c == Color::White {
            self.white_pawns.iter()
        } else {
            self.black_pawns.iter()
        }
    }
    pub fn iter_pieces(&self, c: Color) -> impl Iterator<Item = &Piece> {
        if c == Color::White {
            self.white_pieces.iter()
        } else {
            self.black_pieces.iter()
        }
    }

    pub fn iter_moves(&self) -> impl Iterator<Item = String> {
        return self.iter_moves_general(true);
    }
    pub fn iter_moves_partial_check(&self) -> impl Iterator<Item = String> {
        return self.iter_moves_general(false);
    }
    pub fn iter_normal_moves_only(&self, complete_check: bool) -> impl Iterator<Item = String> {
        let mut sol: Vec<String> = vec![];
        let c = self.turn;
        for &piece_to_move in self.iter().filter(|&p| p.position != None && p.color == c) {
            for relative in piece_to_move.get_relative_moves() {
                sol.extend(Movement::from_relative(&piece_to_move, relative));
            }
        }

        let cloned_board = self.clone();
        return sol
            .into_iter()
            .filter(move |mv| cloned_board.check_move_general(mv, complete_check).is_ok());
    }

    fn iter_moves_general(&self, complete_check: bool) -> impl Iterator<Item = String> {
        //all drops chain all moves filter check_move
        let mut sol: Vec<String> = vec![];

        //drop moves
        for i in 0..80 {
            for piece_to_drop in self
                .iter()
                .filter(|p| p.position == None && p.color == self.turn)
            {
                let mv = Movement {
                    piecetype: piece_to_drop.piecetype,
                    start: None,
                    end: Position(i),
                    promotion: false,
                    force_capture: false,
                    offer_draw: false,
                    withdraw: false,
                    restart: false,
                };
                sol.extend(once(mv.to_string()));
            }
        }

        for piece_to_move in self
            .iter()
            .filter(|p| p.position != None && p.color == self.turn)
        {
            for relative in piece_to_move.get_relative_moves() {
                sol.extend(Movement::from_relative(piece_to_move, relative));
            }
        }

        let cloned_board = self.clone();
        return sol
            .into_iter()
            .filter(move |mv| cloned_board.check_move_general(mv, complete_check).is_ok());
    }
}

#[cfg(test)]
mod test {

    use crate::board::*;
    use crate::movement::*;
    use crate::piece::*;
    use crate::position::*;
    #[test]
    fn play_a_move() {
        let mut b1 = Board::empty();
        let p1: Position = "1f".parse().unwrap();
        b1.add_piece(Piece {
            color: Color::White,
            piecetype: PieceType::Pawn,
            promoted: false,
            position: Some(p1),
        });

        let b3 = b1.play_move("P1f-1g");

        let mut b2 = Board::empty();
        b2.turn.invert();
        let p2: Position = "1g".parse().unwrap();
        b2.add_piece(Piece {
            color: Color::White,
            piecetype: PieceType::Pawn,
            promoted: false,
            position: Some(p2),
        });
        assert_eq!(b2, b3);
    }

    #[test]
    fn play_a_drop() {
        let mut b1 = Board::empty();
        b1.add_piece(Piece {
            color: Color::White,
            piecetype: PieceType::Pawn,
            promoted: false,
            position: None,
        });

        let b3 = b1.play_move("P*3e");

        let mut b2 = Board::empty();
        b2.turn.invert();
        let p2: Position = "3e".parse().unwrap();
        b2.add_piece(Piece {
            color: Color::White,
            piecetype: PieceType::Pawn,
            promoted: false,
            position: Some(p2),
        });

        assert_eq!(b2, b3);
    }
}
