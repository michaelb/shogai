use crate::movement::*;
use crate::piece::*;
use crate::position::*;

#[derive(Debug, Clone)]
pub struct Board {
    piece_set: Vec<Piece>,
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

impl Board {
    fn empty() -> Self {
        Board {
            piece_set: Vec::new(),
        }
    }
    fn play_move(&self, mv: &str) -> Board {
        self.clone()
    }

    fn check_move(&self, mv: &str) -> bool {
        false
    }

    fn is_occupied_by(&self, pos: Position) -> Option<Piece> {
        for &p in self.piece_set.iter() {
            if p.position == Some(pos) {
                return Some(p);
            }
        }
        None
    }

    fn add_piece(&mut self, piece: Piece) {
        self.piece_set.push(piece);
    }

    ///return the list of the piece
    pub fn export(&self) -> Vec<Piece> {
        return self.piece_set.clone();
    }

    ///a simple reverse is not enough since
    ///a central symmetry is needed
    pub fn flip(&mut self) {
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
        self.piece_set = tmp;
    }

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
        b.set(Color::Black);
        b.flip();
        b.set(Color::White);
        b
    }
}
