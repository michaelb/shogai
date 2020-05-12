use crate::piece::*;
use crate::position::*;

#[derive(Debug)]
pub struct Board {
    piece_set: Vec<Piece>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " 9 8 7 6 5 4 3 2 1 \n")?;
        write!(f, "+------------------+\n")?;
        for line in (0..9) {
            write!(f, "|")?;
            for column in (0..9) {
                if let Some(p) = self.is_occupied_by(Position(line * 9 + column)) {
                    write!(f, "{}", p)?;
                } else {
                    write!(f, "  ")?;
                }
            }
            write!(f, "|\n");
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

    pub fn new<'a>() -> Board {
        let mut b = Board::empty();
        for i in (18..27) {
            let p = Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn,
                promoted: false,
                position: Some(Position(i)),
            };
            b.add_piece(p);
        }
        for i in (54..63) {
            let p = Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn,
                promoted: false,
                position: Some(Position(i)),
            };
            b.add_piece(p);
        }

        b
    }
}
