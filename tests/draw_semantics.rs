//! Phase 7.1 draw-semantics regression tests.
//!
//! Only fix (a) — rule-50/mate precedence — survived Phase 7.1: clock >= 100
//! is a draw only if the position is not checkmate (a mate delivered on the
//! 100th-clock move wins). It is bench-identical to the accepted head and
//! fires ~never, so it ships without an SPRT.
//!
//! The other three parts (null clock, cross-null repetition fence,
//! root-aware repetition) collectively lost two `[-3,3]` SPRTs (−7.21, then
//! −11.91 with the root-aware part removed) and were reverted — the whole
//! repetition/null-clock rework is anti-Elo at Rarog's level (PLAN.md lesson
//! 14). The aggressive twofold-is-a-draw behavior it tried to change is
//! kept, and asserted below.

use rarog::board::{Board, GameResult, Move};
use rarog::eval::MATE_SCORE;
use rarog::search::{SearchEvent, Searcher};
use rarog::search_options::SearchOptions;

/// Find the legal move with the given UCI string.
fn mv(board: &Board, uci: &str) -> Move {
    board
        .generate_legal_moves()
        .into_iter()
        .find(|m| m.to_string() == uci)
        .unwrap_or_else(|| panic!("move {uci} should be legal"))
}

fn search_to_depth(board: Board, depth: u32) -> (i32, String) {
    let mut searcher = Searcher::default();
    let mut options = SearchOptions::default();
    options.position.board = board.clone();
    options.limits.depth = Some(depth);
    let result = searcher.search(board, &options, false, || SearchEvent::None);
    (result.score, result.bestmove.to_string())
}

// ---------------------------------------------------------------- (a) ----

#[test]
fn mate_on_the_100th_clock_move_beats_the_rule50_draw() {
    // Ra8# is a quiet rook move: it pushes the clock from 99 to exactly 100.
    let mut board = Board::from_fen("6k1/5ppp/8/8/8/8/8/R5K1 w - - 99 80").unwrap();
    board.make_move(mv(&board, "a1a8"));
    assert_eq!(board.halfmove_clock, 100);
    assert_eq!(board.game_result(), Some(GameResult::WhiteCheckmates));
    assert!(!board.can_declare_draw());
    assert!(!board.can_declare_draw_in_search());
}

#[test]
fn check_with_an_escape_at_clock_100_is_still_a_draw() {
    // Same mating pattern but g7 is empty, so Kg7 escapes: the rule-50 claim
    // stands (only checkmate outranks it).
    let mut board = Board::from_fen("6k1/5p1p/8/8/8/8/8/R5K1 w - - 99 80").unwrap();
    board.make_move(mv(&board, "a1a8"));
    assert_eq!(board.halfmove_clock, 100);
    assert_eq!(board.game_result(), Some(GameResult::Draw));
    assert!(board.can_declare_draw_in_search());
}

#[test]
fn stalemate_at_clock_100_is_a_draw() {
    let board = Board::from_fen("7k/5Q2/6K1/8/8/8/8/8 b - - 100 80").unwrap();
    assert_eq!(board.game_result(), Some(GameResult::Draw));
    assert!(board.can_declare_draw_in_search());
}

#[test]
fn search_finds_mate_in_one_at_clock_99() {
    // Pre-7.1 the mating child (clock 100) returned draw 0 from negamax's
    // rule-50 check, so the search scored the position ~cp 0.
    let board = Board::from_fen("6k1/5ppp/8/8/8/8/8/R5K1 w - - 99 80").unwrap();
    let (score, bestmove) = search_to_depth(board, 4);
    assert_eq!(bestmove, "a1a8");
    assert!(
        score >= MATE_SCORE - 100,
        "expected a mate score, got {score}"
    );
}

// -------------------- aggressive twofold heuristic (7.1d reverted) --------

#[test]
fn a_single_prior_twofold_is_a_search_draw() {
    // The kept heuristic: one prior occurrence within the scan bound scores a
    // draw in search, whether or not it predates the search root. The
    // root-aware variant that suppressed the pre-root case (7.1d) lost 7 Elo
    // and was reverted (lesson 14).
    let mut board = Board::from_fen("6k1/5ppp/8/8/8/8/8/R5K1 w - - 10 40").unwrap();
    board.make_move(mv(&board, "a1a2"));
    board.make_move(mv(&board, "g8f8"));
    board.make_move(mv(&board, "a2a1"));
    board.make_move(mv(&board, "f8g8"));
    assert!(board.can_declare_draw_in_search());
    // The arbiter's threefold rule is unaffected: twice is not three times.
    assert!(!board.can_declare_draw());
}

#[test]
fn three_occurrences_are_a_threefold_for_the_arbiter() {
    let mut board = Board::from_fen("6k1/5ppp/8/8/8/8/8/R5K1 w - - 10 40").unwrap();
    for _ in 0..2 {
        board.make_move(mv(&board, "a1a2"));
        board.make_move(mv(&board, "g8f8"));
        board.make_move(mv(&board, "a2a1"));
        board.make_move(mv(&board, "f8g8"));
    }
    assert!(board.can_declare_draw_in_search());
    assert!(board.can_declare_draw());
}
