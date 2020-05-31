use crate::invalidmoveerror::*;
use crate::movement::*;
use crate::piece::*;
use crate::position::*;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    piece_set: Vec<Piece>,
    turn: Color,
    pub rules: Rules,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " 9 8 7 6 5 4 3 2 1 \n")?;
        write!(f, "+------------------+\n")?;
        for line in 0..9 {
            write!(f, "|")?;
            for column in 0..9 {
                if let Some(p) = self.is_occupied_by(Position(line * 9 + (8 - column))) {
                    write!(f, "{}", p)?;
                } else {
                    write!(f, "  ")?;
                }
            }
            write!(f, "|\n")?;
        }
        write!(f, "+------------------+\n")?;
        Ok(())
    }
}

impl IntoIterator for Board {
    type Item = Piece;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.piece_set.into_iter()
    }
}

impl Board {
    pub fn empty() -> Self {
        Board {
            piece_set: Vec::new(),
            turn: Color::White,
            rules: Rules::default(),
        }
    }
    pub fn get_color(&self) -> Color {
        return self.turn;
    }

    ///capture a piece if there, do nothing else
    fn capture_piece(&mut self, p: Position) {
        if let Some((index, _)) = self
            .piece_set
            .iter()
            .enumerate()
            .find(|(_, piece)| piece.position == Some(p))
        {
            self.piece_set[index].color.invert();
            self.piece_set[index].position = None;
            self.piece_set[index].promoted = false;
        }
    }
    pub fn play_move(&self, mv: &str) -> Board {
        if let Err(e) = self.check_move(mv) {
            //move not valid
            panic!("Invalid movement : {}", e);
        }
        let mut new_board = self.clone();
        let movement: Movement = mv.parse().unwrap();

        //movement was checked so it's ok to just play
        if movement.start != None {
            // the movement is a normal movement
            //
            //if a piece (an opponent's) is here at the destination, remove it, change its color,
            new_board.capture_piece(movement.end);

            //then move the piece
            let index = new_board
                .piece_set
                .iter()
                .enumerate()
                .find(|(_, piece)| piece.position == movement.start)
                .unwrap()
                .0;
            new_board.piece_set[index].position = Some(movement.end);
            new_board.piece_set[index].promoted |= movement.promotion;
        } else {
            // the movement is a drop
            let index = new_board
                .piece_set
                .iter()
                .enumerate()
                .find(|(_, piece)| piece.piecetype == movement.piecetype)
                .unwrap()
                .0;
            new_board.piece_set[index].position = Some(movement.end);
        }
        new_board.turn.invert();
        new_board
    }

    pub fn check_move<'a>(&self, mv: &'a str) -> Result<&'a str, InvalidMoveError> {
        // checks should be performed in this order
        Ok(mv)
            .and_then(check_syntax)
            .and_then(check_in_board)
            .and_then(|mv| check_destination(mv, self.clone()))
            .and_then(|mv| check_start(mv, self.clone()))
            .and_then(|mv| check_possible_move(mv, self.clone()))
            .and_then(|mv| check_nifu(mv, self.clone()))
            .and_then(|mv| check_move_possible_after_drop(mv, self.clone()))
            .and_then(|mv| check_promotion(mv, self.clone()))
            .and_then(|mv| check_uncover_check(mv, self.clone()))
            .and_then(|mv| check_checkmate_by_pawn_drop(mv, self.clone()))
    }

    pub fn is_occupied_by(&self, pos: Position) -> Option<Piece> {
        for &p in self.piece_set.iter() {
            if p.position == Some(pos) {
                return Some(p);
            }
        }
        None
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.piece_set.push(piece);
    }

    ///return the list of the piece
    pub fn export(&self) -> Vec<Piece> {
        return self.piece_set.clone();
    }

    ///a simple reverse is not enough since
    ///a central symmetry is needed
    pub fn flip(&mut self) -> Self {
        let mut tmp: Vec<Piece> = Vec::new();
        for piece in self.piece_set.iter() {
            let pos = piece.position;
            if let Some(x) = pos {
                let i = x.0 % 9;
                let j = x.0 / 9;
                let new_x = (8 - j) * 9 + (8 - i);
                tmp.push(Piece {
                    color: piece.color,
                    piecetype: piece.piecetype,
                    promoted: piece.promoted,
                    position: Some(Position(new_x)),
                });
            }
        }
        Board {
            piece_set: tmp,
            turn: self.turn,
            rules: self.rules,
        }
    }

    ///set the regular starting position for one player
    fn set(&mut self, col: Color) {
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
        b.set(Color::White);
        let mut b2 = b.flip();
        b2.set(Color::Black);
        b2
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

        let b3 = b1.play_move("P1f-1g+");

        let mut b2 = Board::empty();
        b2.turn.invert();
        let p2: Position = "1g".parse().unwrap();
        b2.add_piece(Piece {
            color: Color::White,
            piecetype: PieceType::Pawn,
            promoted: true,
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
