use shakmaty::{Chess, Move, Role, Position, attacks::pawn_attacks, Piece, Square, Color};

type PieceScore = i32;

pub fn move_score(pos: &Chess, m: &Move) -> PieceScore {
    let promotion_score = promotion_score(m); 
    let capture_score = capture_score(m); 
    let pawn_defense_score = pawn_defense_score(pos, m); 
    let move_value = move_pst_score(pos.turn(), m);

    let to_value = PieceScore::from(m.to());
    let from_value = PieceScore::from(m.from().expect("No from square"));

    (promotion_score << 26) +
    (capture_score << 25) +
    (pawn_defense_score << 24) +
    (move_value << 12) +
    (to_value << 6) +
    from_value
}

/// Calculate the score for a move that promotes a pawn
/// 0: No promotion
/// 1: Knight
/// 2: Bishop
/// 3: Rook
/// 4: Queen
fn promotion_score(m: &Move) -> PieceScore {
    PieceScore::from(m.promotion().unwrap_or(Role::Pawn)) - 1
}

/// Calculate the score for a move that captures a piece
/// 0: No capture
/// 1: Capture
fn capture_score(m: &Move) -> PieceScore {
    PieceScore::from(m.is_capture())
}

/// Calculate the score for a move that may be attacked by an opponent pawn
fn pawn_defense_score(pos: &Chess, m: &Move) -> PieceScore {
    // possible opponent pawn squares that can attack the player's destination square
    let pawn_attack_squares = pawn_attacks(pos.turn(), m.to());

    // all pawn squares on the board
    let pawn_squares = pos.board().pawns();

    // all squares occupied by the opponent
    let opponent_squares = pos.them();

    // AND the bitboards together to get the opponent pawn squares that can attack the player's destination square
    let defended_squares = pawn_attack_squares & pawn_squares & opponent_squares;

    // if there are any defended squares, subtract the move role score from 6
    if defended_squares.any() {
        6 - PieceScore::from(m.role())
    } else {
        6
    }
}

/// Calculate the score for a piece according to Lichess piece square tables
fn pst_score(piece: Piece, square: Square) -> PieceScore {
    let sq = if piece.color.is_white() {
        square.flip_vertical()
    } else {
        square
    };
    PieceScore::from(LICHESS_TABLES[piece.role as usize - 1][sq as usize])
}

/// Calculate the score for a move according to Lichess piece square tables
/// Add 512 to the score to make it positive
fn move_pst_score(turn: Color, m: &Move) -> PieceScore {
    let to_score = pst_score(m.role().of(turn), m.to());
    let from_score = pst_score(m.role().of(turn), m.from().expect("No from square"));
    512 + to_score - from_score
}

#[rustfmt::skip]
const LICHESS_TABLES: [[i16; 64]; 6] = [
    [   0,  0,  0,  0,  0,  0,  0,  0,
       50, 50, 50, 50, 50, 50, 50, 50,
       10, 10, 20, 30, 30, 20, 10, 10,
        5,  5, 10, 25, 25, 10,  5,  5,
        0,  0,  0, 20, 21,  0,  0,  0,
        5, -5,-10,  0,  0,-10, -5,  5,
        5, 10, 10,-31,-31, 10, 10,  5,
        0,  0,  0,  0,  0,  0,  0,  0],

    [ -50,-40,-30,-30,-30,-30,-40,-50,
      -40,-20,  0,  0,  0,  0,-20,-40,
      -30,  0, 10, 15, 15, 10,  0,-30,
      -30,  5, 15, 20, 20, 15,  5,-30,
      -30,  0, 15, 20, 20, 15,  0,-30,
      -30,  5, 10, 15, 15, 11,  5,-30,
      -40,-20,  0,  5,  5,  0,-20,-40,
      -50,-40,-30,-30,-30,-30,-40,-50],

    [ -20,-10,-10,-10,-10,-10,-10,-20,
      -10,  0,  0,  0,  0,  0,  0,-10,
      -10,  0,  5, 10, 10,  5,  0,-10,
      -10,  5,  5, 10, 10,  5,  5,-10,
      -10,  0, 10, 10, 10, 10,  0,-10,
      -10, 10, 10, 10, 10, 10, 10,-10,
      -10,  5,  0,  0,  0,  0,  5,-10,
      -20,-10,-10,-10,-10,-10,-10,-20],

    [   0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10, 10, 10, 10, 10,  5,
       -5,  0,  0,  0,  0,  0,  0, -5,
       -5,  0,  0,  0,  0,  0,  0, -5,
       -5,  0,  0,  0,  0,  0,  0, -5,
       -5,  0,  0,  0,  0,  0,  0, -5,
       -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  5,  5,  0,  0,  0],

    [ -20,-10,-10, -5, -5,-10,-10,-20,
      -10,  0,  0,  0,  0,  0,  0,-10,
      -10,  0,  5,  5,  5,  5,  0,-10,
       -5,  0,  5,  5,  5,  5,  0, -5,
        0,  0,  5,  5,  5,  5,  0, -5,
      -10,  5,  5,  5,  5,  5,  0,-10,
      -10,  0,  5,  0,  0,  0,  0,-10,
      -20,-10,-10, -5, -5,-10,-10,-20],

    [ -30,-40,-40,-50,-50,-40,-40,-30,
      -30,-40,-40,-50,-50,-40,-40,-30,
      -30,-40,-40,-50,-50,-40,-40,-30,
      -30,-40,-40,-50,-50,-40,-40,-30,
      -20,-30,-30,-40,-40,-30,-30,-20,
      -10,-20,-20,-20,-20,-20,-20,-10,
       20, 20,  0,  0,  0,  0, 20, 20,
        0, 30, 10,  0,  0, 10, 30,  0]
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that the score for a move that promotes to a Knight is 1
    fn knight_promotion_score() {
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::A7,
            to: Square::B8,
            capture: None,
            promotion: Some(Role::Knight),
        }; 
        assert_eq!(promotion_score(&m), 1);
    }

    #[test]
    /// Test that the score for a move that promotes to a Bishop is 2
    fn bishop_promotion_score() {
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::A7,
            to: Square::B8,
            capture: None,
            promotion: Some(Role::Bishop),
        }; 
        assert_eq!(promotion_score(&m), 2);
    }

    #[test]
    /// Test that the score for a move that promotes to a Rook is 3
    fn rook_promotion_score() {
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::A7,
            to: Square::B8,
            capture: None,
            promotion: Some(Role::Rook),
        }; 
        assert_eq!(promotion_score(&m), 3);
    }

    #[test]
    /// Test that the score for a move that promotes to a Queen is 4
    fn queen_promotion_score() {
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::A7,
            to: Square::B8,
            capture: None,
            promotion: Some(Role::Queen),
        }; 
        assert_eq!(promotion_score(&m), 4);
    }

    #[test]
    /// Tests that a move that captures a piece has a capture score of 1
    fn capture_score_test() {
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::A7,
            to: Square::B8,
            capture: Some(Role::Knight),
            promotion: None,
        }; 
        assert_eq!(capture_score(&m), 1);
    }

    #[test]
    /// Tests that a move that does not capture a piece has a capture score of 0
    fn no_capture_score_test() {
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::A7,
            to: Square::B8,
            capture: None,
            promotion: None,
        }; 
        assert_eq!(capture_score(&m), 0);
    }

    #[test]
    /// Tests that a move that results in a pawn being attacked by an opponent pawn has a pawn defense score of 5
    fn pawn_defense_score_test() {
        let pos = Chess::default();
        let white_move = Move::Normal {
            role: Role::Pawn,
            from: Square::A2,
            to: Square::A4,
            capture: None,
            promotion: None,
        }; 
        let pos = pos.play(&white_move).expect("Move is illegal");
        let black_move = Move::Normal {
            role: Role::Pawn,
            from: Square::B7,
            to: Square::B5,
            capture: None,
            promotion: None,
        };
        assert_eq!(pawn_defense_score(&pos, &black_move), 5);
    }

    #[test]
    /// Tests that the pst score for a white pawn on B2 is 10
    fn white_pawn_pst_score_test() {
        let pos = Chess::default();
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::B2,
            to: Square::B4,
            capture: None,
            promotion: None,
        }; 
        assert_eq!(pst_score(m.role().of(pos.turn()), m.from().expect("No from square")), 10);
    }

    #[test]
    /// Tests that the pst score for a white pawn move from B2 to B3 is 512 + (-5) - (10)
    fn white_pawn_move_pst_score_test() {
        let pos = Chess::default();
        let m = Move::Normal {
            role: Role::Pawn,
            from: Square::B2,
            to: Square::B3,
            capture: None,
            promotion: None,
        }; 
        assert_eq!(move_pst_score(pos.turn(), &m), 512 - 5 - 10);
    }

}