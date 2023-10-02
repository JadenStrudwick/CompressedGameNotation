
use tanton::{BitMove};
use tanton::core::piece_move::{PreMoveInfo, MoveFlag};
use tanton::core::sq::{SQ};
use crate::pgn::{SanPlusWrapper};
use pgn_reader::{Visitor, SanPlus, San};
use shakmaty::{Chess, Move};
use tanton;

pub fn san_plus_wrapper_to_bit_move(board: &Chess, san_plus: &SanPlusWrapper) -> BitMove {

    let mov = &san_plus.0.san;
    let mov2 = mov.to_move(board).unwrap();

    let from = mov2.from().unwrap();
    let fromSQ = shakmaty_square_to_tanton_square(from);

    let to = mov2.to();
    let toSQ = shakmaty_square_to_tanton_square(to);

    let mut flags = MoveFlag::QuietMove;

    // construct moveflags
    if mov2.is_promotion() {
        let promotion_piece = shakmaty_role_to_plece_piece(mov2.promotion().unwrap());
        flags = MoveFlag::Promotion { capture: mov2.is_capture(), prom: promotion_piece };
    } else if mov2.is_castle() {
        let castle_side = mov2.castling_side().unwrap();
        if castle_side == shakmaty::CastlingSide::KingSide {
            flags = MoveFlag::Castle { king_side: true };
        } else {
            flags = MoveFlag::Castle { king_side: false }; 
        }
    } else if is_double_pawn_push(&mov2) {
        flags = MoveFlag::DoublePawnPush;
    } else if mov2.is_capture() {
        let en_passant = mov2.is_en_passant();
        if en_passant {
            flags = MoveFlag::Capture { ep_capture: true };
        } else {
            flags = MoveFlag::Capture { ep_capture: false };
        }
    }

    let premove_info = PreMoveInfo {
        src: fromSQ,
        dst: toSQ,
        flags: flags,
    };

    return BitMove::init(premove_info);
    
}

fn shakmaty_file_to_tanton_file(file: shakmaty::File) -> tanton::core::File {
    match file {
        shakmaty::File::A => tanton::core::File::A,
        shakmaty::File::B => tanton::core::File::B,
        shakmaty::File::C => tanton::core::File::C,
        shakmaty::File::D => tanton::core::File::D,
        shakmaty::File::E => tanton::core::File::E,
        shakmaty::File::F => tanton::core::File::F,
        shakmaty::File::G => tanton::core::File::G,
        shakmaty::File::H => tanton::core::File::H,
    }
}

fn shakmaty_rank_to_tanton_rank(rank: shakmaty::Rank) -> tanton::core::Rank {
    match rank {
        shakmaty::Rank::First => tanton::core::Rank::R1,
        shakmaty::Rank::Second => tanton::core::Rank::R2,
        shakmaty::Rank::Third => tanton::core::Rank::R3,
        shakmaty::Rank::Fourth => tanton::core::Rank::R4,
        shakmaty::Rank::Fifth => tanton::core::Rank::R5,
        shakmaty::Rank::Sixth => tanton::core::Rank::R6,
        shakmaty::Rank::Seventh => tanton::core::Rank::R7,
        shakmaty::Rank::Eighth => tanton::core::Rank::R8,
    }
}

fn shakmaty_square_to_tanton_square(square: shakmaty::Square) -> tanton::core::sq::SQ {
    let file = shakmaty_file_to_tanton_file(square.file());
    let rank = shakmaty_rank_to_tanton_rank(square.rank());
    SQ::make(file, rank)
}

fn shakmaty_role_to_plece_piece(role: shakmaty::Role) -> tanton::core::PieceType {
    match role {
        shakmaty::Role::King => tanton::core::PieceType::K,
        shakmaty::Role::Queen => tanton::core::PieceType::Q,
        shakmaty::Role::Rook => tanton::core::PieceType::R,
        shakmaty::Role::Bishop => tanton::core::PieceType::B,
        shakmaty::Role::Knight => tanton::core::PieceType::N,
        shakmaty::Role::Pawn => tanton::core::PieceType::P,
    }
}

fn is_double_pawn_push(mov: &Move) -> bool {
    let piece = mov.role();
    if piece == shakmaty::Role::Pawn {
        let from = mov.from().unwrap();
        let to = mov.to();
        
        let fromSQ = shakmaty_square_to_tanton_square(from);
        let toSQ = shakmaty_square_to_tanton_square(to);

        // check if the move is a double pawn push
        if from.rank() == shakmaty::Rank::Second && to.rank() == shakmaty::Rank::Fourth {
            return true;
        } else if from.rank() == shakmaty::Rank::Seventh && to.rank() == shakmaty::Rank::Fifth {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

use tanton::board::Board;

fn eval_move(board: &mut Board, mov: BitMove) -> tanton::core::score::Value {
    let mut temp_board = board.clone();
    temp_board.apply_move(mov);
    tanton::tools::eval::Eval::eval_low(&temp_board)
}

pub fn eval_legal_moves(board: &mut Board) -> Vec<(BitMove, tanton::core::score::Value)> {
    let mut moves = Vec::new();
    let legal_moves = board.generate_moves();
    for mov in legal_moves {
        let score = eval_move(board, mov);
        moves.push((mov, score));
    }
    return moves;
}