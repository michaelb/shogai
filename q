correctly manage promoted piece possible moves
# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch master
# Your branch is up to date with 'origin/master'.
#
# Changes to be committed:
#       modified:   src/piece.rs
#
# ------------------------ >8 ------------------------
# Do not modify or remove the line above.
# Everything below it will be ignored.
diff --git a/src/piece.rs b/src/piece.rs
index 76ebe14..2c275ca 100644
--- a/src/piece.rs
+++ b/src/piece.rs
@@ -55,7 +55,7 @@ impl Piece {
  //check position % 9 == (position + relat_mov%9)%9

  //also, whether the piece has to jump over other piece is not checked!
  -        let possibles_moves: Vec<i32> = match &self.piecetype {
    +        let mut possibles_moves: Vec<i32> = match &self.piecetype {
      PieceType::Pawn => vec![9],
      PieceType::King => vec![1, -1, 10, 8, 9, -9, -8, -10],
      PieceType::Rook => vec![
        @@ -71,6 +71,22 @@ impl Piece {
          PieceType::Knight => vec![17, 19],
          PieceType::Lance => vec![9, 18, 27, 36, 45, 54, 63, 72],
        };
      +
        +        //manage promotion
        +        if self.promoted {
          +            if self.piecetype == PieceType::Pawn
            +                || self.piecetype == PieceType::Lance
            +                || self.piecetype == PieceType::Knight
            +                || self.piecetype == PieceType::Lance
            +            {
              +                possibles_moves = vec![1, -1, 8, 9, 10, -9]
                +            } else {
                  +                possibles_moves.append(vec![1, -1, 8, 9, 10, -8, -9, -10]);
                  +                possibles_moves.sort();
                  +                possibles_moves.dedup();
                  +            }
          +        }
      +
        let possibles_moves_colored;
      if self.color == Color::White {
        possibles_moves_colored = possibles_moves;
