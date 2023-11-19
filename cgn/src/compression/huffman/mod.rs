mod codes;
mod score_move;

use score_move::move_score;
use shakmaty::{Chess, Move, Position};

/// Generate legal moves for a given position
fn generate_legal_moves(pos: &Chess) -> Vec<Move> {
    let mut moves = Vec::new();
    for m in pos.legal_moves() {
        moves.push(m);
    }
    moves
}

/// Order moves by score
fn order_legal_moves(pos: &Chess, moves: &mut Vec<Move>) -> Vec<Move> {
    moves.sort_by_cached_key(|m| move_score(pos, m));
    moves.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_legal_moves() { 
        let pos = Chess::default();
        let moves = generate_legal_moves(&pos);
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_order_legal_moves() {
        let pos = Chess::default();
        let mut moves = generate_legal_moves(&pos);
        let ordered_moves = order_legal_moves(&pos, &mut moves);
        println!("{:?}", ordered_moves);
        assert_eq!(ordered_moves.len(), 20);
    }
}
