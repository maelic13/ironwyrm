//! Native adapter exposing Rarog 2.3.2's HCE to the pre-NNUE Stockfish search.

use std::cell::RefCell;

use rarog::board::{Board, CastlingRights, Color};
use rarog::eval::Evaluator;

const ABI_VERSION: u32 = 1;

thread_local! {
    /// Stockfish owns one searcher per OS thread. Matching that ownership here
    /// keeps Rarog's mutable pawn/eval caches private without locks.
    static EVALUATOR: RefCell<Evaluator> = RefCell::new(Evaluator::default());
}

/// ABI handshake checked when the hybrid loads the DLL.
#[unsafe(no_mangle)]
pub extern "C" fn rarog_hce_abi_version() -> u32 {
    ABI_VERSION
}

/// Evaluate a legal Stockfish position with Rarog's shipped HCE.
///
/// # Safety
///
/// `pieces` must point to twelve readable `u64`s in
/// `[WP..WK, BP..BK]` order. Stockfish supplies legal, non-overlapping
/// bitboards. Invalid metadata returns `i32::MIN` and is treated as fatal by
/// the C++ caller rather than allowing a corrupt score into search.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rarog_hce_evaluate(
    pieces: *const u64,
    side_to_move: u8,
    castling: u8,
    halfmove_clock: u8,
) -> i32 {
    if pieces.is_null() {
        return i32::MIN;
    }
    let side = match side_to_move {
        0 => Color::White,
        1 => Color::Black,
        _ => return i32::MIN,
    };
    // SAFETY: the caller contract above requires twelve readable u64 values.
    let input = unsafe { std::slice::from_raw_parts(pieces, 12) };
    let mut snapshot = [0u64; 12];
    snapshot.copy_from_slice(input);
    let Ok(board) =
        Board::from_eval_snapshot(snapshot, side, CastlingRights(castling), halfmove_clock)
    else {
        return i32::MIN;
    };
    EVALUATOR.with_borrow_mut(|evaluator| evaluator.evaluate(&board))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rarog::board::Piece;

    fn snapshot(board: &Board) -> [u64; 12] {
        let mut pieces = [0u64; 12];
        for color in [Color::White, Color::Black] {
            for piece in Piece::ALL {
                pieces[color as usize * 6 + piece as usize] = board.pieces(color, piece).0;
            }
        }
        pieces
    }

    #[test]
    fn snapshot_reconstructs_shipped_hce_exactly() {
        let fens = [
            rarog::board::STARTING_FEN,
            "r3k2r/ppp2ppp/2n1bn2/3qp3/3P4/2N1BN2/PPP1QPPP/R3K2R w KQkq - 7 12",
            "8/5pk1/6p1/3P3p/4P3/5K2/8/8 b - - 73 54",
            "8/2p5/2P5/3K4/8/8/6k1/8 w - - 99 80",
        ];
        for fen in fens {
            let original = Board::from_fen(fen).expect("valid test FEN");
            let rebuilt = Board::from_eval_snapshot(
                snapshot(&original),
                original.side_to_move(),
                original.castling,
                original.halfmove_clock,
            )
            .expect("valid snapshot");
            let expected = Evaluator::default().evaluate(&original);
            let actual = Evaluator::default().evaluate(&rebuilt);
            assert_eq!(actual, expected, "snapshot mismatch for {fen}");

            let exported = unsafe {
                rarog_hce_evaluate(
                    snapshot(&original).as_ptr(),
                    original.side_to_move() as u8,
                    original.castling.0,
                    original.halfmove_clock,
                )
            };
            assert_eq!(exported, expected, "ABI mismatch for {fen}");
        }
    }
}
