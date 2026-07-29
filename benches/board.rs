use std::hint::black_box;
use std::time::{Duration, Instant};

use rarog::board::{Board, generate_captures, generate_legal_moves, perft};
use rarog::eval::Evaluator;

const WARMUP: Duration = Duration::from_millis(150);
// 9.7: N shorter samples instead of one 750 ms shot. A single sample on a
// desktop is hostage to whatever the OS scheduler did during those 750 ms —
// we have measured several-percent swings on IDENTICAL binaries — so one
// number cannot distinguish a real 2% change from noise. The median resists
// scheduler outliers; the MAD is printed beside it so the output itself says
// whether a difference between two runs is resolvable or inside the noise.
const SAMPLES: usize = 9;
const SAMPLE_TIME: Duration = Duration::from_millis(150);

const BENCHMARK_FENS: &[(&str, &str)] = &[
    (
        "startpos",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    ),
    (
        "kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ),
    (
        "midgame",
        "rnbq1k1r/pppp1ppp/4pn2/8/1b1PP3/2N2N2/PPP2PPP/R1BQKB1R w KQ - 2 5",
    ),
    ("endgame", "8/2p5/3p4/KP5r/8/8/8/7k w - - 0 1"),
    (
        "in-check",
        "rnbqkb1r/pppp1ppp/5n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 3 3",
    ),
];

struct BenchResult {
    label: &'static str,
    unit: &'static str,
    /// Per-sample throughput (ops/s), one entry per sample, unsorted.
    samples: Vec<f64>,
    iterations: u64,
}

impl BenchResult {
    /// Median throughput across samples — the robust point estimate.
    fn median(&self) -> f64 {
        let mut sorted = self.samples.clone();
        sorted.sort_by(f64::total_cmp);
        sorted[sorted.len() / 2]
    }

    /// Median absolute deviation, same units as the median. A rough 1-sigma
    /// analogue that ignores scheduler outliers entirely.
    fn mad(&self) -> f64 {
        let med = self.median();
        let mut dev: Vec<f64> = self.samples.iter().map(|s| (s - med).abs()).collect();
        dev.sort_by(f64::total_cmp);
        dev[dev.len() / 2]
    }

    /// MAD as a percentage of the median — the at-a-glance noise figure.
    fn spread_pct(&self) -> f64 {
        100.0 * self.mad() / self.median()
    }
}

fn main() {
    let boards: Vec<Board> = BENCHMARK_FENS
        .iter()
        .map(|(_, fen)| Board::from_fen(fen).unwrap())
        .collect();

    let mut capture_boards = boards.clone();
    let mut mutable_boards = boards.clone();
    let mut see_boards = boards.clone();
    let mut simulation_boards = boards.clone();
    let mut evaluator = Evaluator::default();

    let results = [
        measure("legal movegen", "moves", || legal_movegen(&boards)),
        measure("legal validation", "moves", || legal_validation(&boards)),
        measure("capture gen", "moves", || capture_gen(&mut capture_boards)),
        measure("make/unmake", "moves", || make_unmake(&mut mutable_boards)),
        measure("check detection", "positions", || check_detection(&boards)),
        measure("see captures", "captures", || see_captures(&mut see_boards)),
        measure("evaluation", "positions", || {
            eval_positions(&boards, &mut evaluator)
        }),
        measure("game simulation", "moves", || {
            game_simulation(&mut simulation_boards)
        }),
        measure("perft startpos d4", "nodes", || perft_startpos(4)),
    ];

    println!();
    println!("Rarog board benchmark");
    println!("positions: {}", BENCHMARK_FENS.len());
    println!("warmup: {} ms", WARMUP.as_millis());
    println!(
        "samples: {SAMPLES} x {} ms per workload (median +- MAD)",
        SAMPLE_TIME.as_millis()
    );
    println!();
    println!(
        "{:<20} {:>16} {:>13} {:<10} {:>12}",
        "workload", "median", "MAD (noise)", "unit", "iterations"
    );
    println!("{}", "-".repeat(78));

    for result in &results {
        println!(
            "{:<20} {:>16.0} {:>9.0} ({:>4.1}%) {:<10} {:>12}",
            result.label,
            result.median(),
            result.mad(),
            result.spread_pct(),
            result.unit,
            result.iterations
        );
    }
}

fn measure<F>(label: &'static str, unit: &'static str, mut workload: F) -> BenchResult
where
    F: FnMut() -> u64,
{
    let warmup_start = Instant::now();
    while warmup_start.elapsed() < WARMUP {
        black_box(workload());
    }

    let mut samples = Vec::with_capacity(SAMPLES);
    let mut iterations = 0u64;
    for _ in 0..SAMPLES {
        let start = Instant::now();
        let mut ops = 0u64;
        while start.elapsed() < SAMPLE_TIME {
            ops += black_box(workload());
            iterations += 1;
        }
        samples.push(ops as f64 / start.elapsed().as_secs_f64());
    }

    BenchResult {
        label,
        unit,
        samples,
        iterations,
    }
}

fn legal_movegen(boards: &[Board]) -> u64 {
    boards
        .iter()
        .map(|board| black_box(generate_legal_moves(black_box(board)).len() as u64))
        .sum()
}

fn legal_validation(boards: &[Board]) -> u64 {
    let mut ops = 0u64;
    for board in boards {
        let moves = generate_legal_moves(board);
        for mv in moves {
            black_box(
                board
                    .legal_move(black_box(mv))
                    .expect("generated move is legal"),
            );
            ops += 1;
        }
    }
    ops
}

fn capture_gen(boards: &mut [Board]) -> u64 {
    boards
        .iter_mut()
        .map(|board| black_box(generate_captures(black_box(board)).len() as u64))
        .sum()
}

fn make_unmake(boards: &mut [Board]) -> u64 {
    let mut ops = 0u64;
    for board in boards {
        let moves = generate_legal_moves(board);
        for mv in moves {
            board.make_move(mv);
            black_box(&board);
            board.unmake_move(mv);
            ops += 1;
        }
    }
    ops
}

fn check_detection(boards: &[Board]) -> u64 {
    boards
        .iter()
        .map(|board| {
            black_box(board.is_in_check());
            1
        })
        .sum()
}

fn see_captures(boards: &mut [Board]) -> u64 {
    let mut ops = 0u64;
    for board in boards {
        let captures = generate_captures(board);
        for &mv in &captures {
            black_box(board.see(mv));
            ops += 1;
        }
    }
    ops
}

fn eval_positions(boards: &[Board], evaluator: &mut Evaluator) -> u64 {
    boards
        .iter()
        .map(|board| {
            black_box(evaluator.evaluate(black_box(board)));
            1
        })
        .sum()
}

fn game_simulation(boards: &mut [Board]) -> u64 {
    let mut ops = 0u64;
    for board in boards {
        let moves = generate_legal_moves(board);
        for mv in moves {
            board.make_move(mv);
            ops += generate_legal_moves(board).len() as u64;
            board.unmake_move(mv);
        }
    }
    ops
}

fn perft_startpos(depth: u32) -> u64 {
    let mut board = Board::starting_position();
    perft(&mut board, depth)
}
