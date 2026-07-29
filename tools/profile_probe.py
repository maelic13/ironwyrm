#!/usr/bin/env python
"""8.12(c) cost attribution: apply ONE duplicate-work probe to the tree.

Behaviour-preserving profiling without a profiler: run region R a second
time, discard the result behind black_box, and measure the NPS drop. If R is
fraction f of runtime, doubling it gives NPS_base/NPS_dup = 1+f, so
    f = NPS_base / NPS_dup - 1
Bench must stay bit-identical (the duplicate's result is thrown away), which
is the correctness check on every probe.

usage: mkprobe.py <region>|revert
"""
import subprocess, sys, pathlib

REPO = pathlib.Path(r"D:\code\rarog")

# region -> (file, anchor_text, replacement_text)
PROBES = {

# --- eval: the expensive non-lazy block -------------------------------------
"eval_activity": ("src/eval.rs",
"""            self.eval_piece_activity(board, atk, &mut mg, &mut eg, &passed, &pawn_attacks, phase);""",
"""            {
                let (mut d_mg, mut d_eg) = (std::hint::black_box(mg), std::hint::black_box(eg));
                self.eval_piece_activity(
                    std::hint::black_box(board), atk, &mut d_mg, &mut d_eg,
                    &passed, &pawn_attacks, std::hint::black_box(phase));
                std::hint::black_box(d_mg + d_eg);
            }
            self.eval_piece_activity(board, atk, &mut mg, &mut eg, &passed, &pawn_attacks, phase);"""),

# --- eval: pawn structure (incl. its cache) ---------------------------------
"eval_pawns": ("src/eval.rs",
"""        let (pawn_mg, pawn_eg) = self.eval_pawns(board, atk, &mut passed, &mut pawn_attacks);""",
"""        {
            let (mut d_p, mut d_a) = ([Bitboard::EMPTY; 2], [Bitboard::EMPTY; 2]);
            let r = self.eval_pawns(std::hint::black_box(board), atk, &mut d_p, &mut d_a);
            std::hint::black_box(r);
        }
        let (pawn_mg, pawn_eg) = self.eval_pawns(board, atk, &mut passed, &mut pawn_attacks);"""),

# --- eval: material + PST + phase walk (the 8.12(a) target) -----------------
"eval_matpst": ("src/eval.rs",
"""        for color in [Color::White, Color::Black] {
            let sign = color_sign(color);
            for piece in Piece::ALL {
                let mut bb = board.pieces(color, piece);""",
"""        for color in [Color::White, Color::Black] {
            let sign = color_sign(color);
            for piece in Piece::ALL {
                {
                    let (mut d_mg, mut d_eg, mut d_ph) = (0i32, 0i32, 0i32);
                    let mut d_bb = std::hint::black_box(board).pieces(color, piece);
                    let w = PHASE_W[piece as usize];
                    while d_bb.any() {
                        let sq = d_bb.pop_lsb();
                        d_ph += w;
                        d_mg += sign * self.tables.mg[color as usize][piece as usize][sq.index()];
                        d_eg += sign * self.tables.eg[color as usize][piece as usize][sq.index()];
                    }
                    std::hint::black_box(d_mg + d_eg + d_ph);
                }
                let mut bb = board.pieces(color, piece);"""),

# --- eval: imbalance --------------------------------------------------------
"eval_imbalance": ("src/eval.rs",
"""            self.eval_imbalance(board, &mut mg, &mut eg);""",
"""            {
                let (mut d_mg, mut d_eg) = (std::hint::black_box(mg), std::hint::black_box(eg));
                self.eval_imbalance(std::hint::black_box(board), &mut d_mg, &mut d_eg);
                std::hint::black_box(d_mg + d_eg);
            }
            self.eval_imbalance(board, &mut mg, &mut eg);"""),

# --- whole eval (upper bound for all eval work) -----------------------------
"eval_total": ("src/search.rs",
"""    fn raw_eval(&mut self, board: &Board) -> i32 {
        self.evaluator.evaluate(board)
    }""",
"""    fn raw_eval(&mut self, board: &Board) -> i32 {
        std::hint::black_box(self.evaluator.evaluate(std::hint::black_box(board)));
        self.evaluator.evaluate(board)
    }"""),

# --- TT probe (negamax) -----------------------------------------------------
"tt_probe": ("src/search.rs",
"""        let tt_entry = self.tt.probe(hash);
        let tt_raw_move = tt_entry.and_then(|entry| entry.best_move());""",
"""        std::hint::black_box(self.tt.probe(std::hint::black_box(hash)));
        let tt_entry = self.tt.probe(hash);
        let tt_raw_move = tt_entry.and_then(|entry| entry.best_move());"""),

# --- staged capture generation + pin computation ----------------------------
"gen_captures": ("src/search.rs",
"""        let (captures, pinned) = board.generate_legal_captures_pinned();""",
"""        std::hint::black_box(std::hint::black_box(&mut *board).generate_legal_captures_pinned());
        let (captures, pinned) = board.generate_legal_captures_pinned();"""),

# --- staged quiet generation ------------------------------------------------
"gen_quiets": ("src/search.rs",
"""                    let quiet_moves = board.generate_legal_quiets_pinned(*pinned);""",
"""                    std::hint::black_box(
                        std::hint::black_box(&*board).generate_legal_quiets_pinned(*pinned));
                    let quiet_moves = board.generate_legal_quiets_pinned(*pinned);"""),

# --- quiet move scoring (history lookups) -----------------------------------
"score_quiets": ("src/search.rs",
"""                    searcher.append_scored_moves(
                        board,
                        quiet_moves.as_slice(),
                        *tt_move,
                        *ply,
                        moves,
                    );""",
"""                    {
                        let mut d = ScoredMoveList::new();
                        searcher.append_scored_moves(
                            std::hint::black_box(board), quiet_moves.as_slice(),
                            *tt_move, *ply, &mut d);
                        std::hint::black_box(d.len());
                    }
                    searcher.append_scored_moves(
                        board,
                        quiet_moves.as_slice(),
                        *tt_move,
                        *ply,
                        moves,
                    );"""),

# --- gives-check predicate (10.3(2)/(3) region) -----------------------------
"gives_check": ("src/search.rs",
"""            let mv_gives_check = move_gives_check(board, &mut node_ci, mv, &mut gives_check);""",
"""            {
                let mut d_ci: Option<CheckInfo> = None;
                let mut d_gc = gives_check;
                std::hint::black_box(move_gives_check(
                    std::hint::black_box(board), &mut d_ci, mv, &mut d_gc));
            }
            let mv_gives_check = move_gives_check(board, &mut node_ci, mv, &mut gives_check);"""),

# --- make/unmake (board mutation cost in search conditions) -----------------
"make_move": ("src/search.rs",
"""            board.make_move_with_check(mv, mv_gives_check);""",
"""            board.make_move_with_check(mv, mv_gives_check);
            board.unmake_move(mv);
            board.make_move_with_check(mv, mv_gives_check);"""),
}


def apply(region):
    f, anchor, repl = PROBES[region]
    p = REPO / f
    s = p.read_text(encoding="utf-8")
    if anchor not in s:
        sys.exit(f"ANCHOR NOT FOUND for {region} in {f}")
    if s.count(anchor) != 1:
        sys.exit(f"anchor for {region} appears {s.count(anchor)}x (need exactly 1)")
    p.write_text(s.replace(anchor, repl), encoding="utf-8", newline="")
    print(f"applied probe: {region} -> {f}")


if __name__ == "__main__":
    arg = sys.argv[1]
    if arg == "revert":
        subprocess.run(["git", "checkout", "--", "src/"], cwd=REPO, check=True)
        print("reverted")
    elif arg == "list":
        print(" ".join(PROBES))
    else:
        apply(arg)
