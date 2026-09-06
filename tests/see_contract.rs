//! External legal-exchange truth and explicit production policy boundaries.
//! Regenerate with tools/diag/see_contract_oracle.py (python-chess).
//! Debt rows are enforced by the ignored acceptance test at 4.11b.5, not
//! redefined as correct just to make the current engine pass.

use rarog::board::Board;

const FIXTURES: &str = include_str!("data/see-contract-v1.tsv");

#[test]
fn report_see_contract_observations() {
    for line in FIXTURES.lines().filter(|line| !line.starts_with('#')) {
        let f: Vec<_> = line.split('|').collect();
        assert_eq!(f.len(), 6);
        let board = Board::from_fen(f[2]).expect("independent FEN is legal");
        let mv = board.parse_move(f[3]).expect("independent move is legal");
        let truth: i32 = f[4].parse().unwrap();
        let thresholds = [truth - 1, truth, truth + 1, 0];
        let ordinary = thresholds.map(|t| board.see_ge(mv, t));
        let quiet_aware = thresholds.map(|t| board.see_ge_quiet_aware(mv, t));
        println!(
            "{}|{}|tree={truth}|see={}|thresholds={thresholds:?}|ge={ordinary:?}|quiet={quiet_aware:?}",
            f[0],
            f[1],
            board.see(mv)
        );
    }
}

fn verify(selected_debt: Option<&str>) {
    let mut checked = 0;
    for line in FIXTURES.lines().filter(|line| !line.starts_with('#')) {
        let f: Vec<_> = line.split('|').collect();
        assert_eq!(f.len(), 6);
        let debt = f[1] == "debt";
        if selected_debt.map_or(debt, |name| name != f[0]) {
            continue;
        }
        checked += 1;
        let board = Board::from_fen(f[2]).expect("valid fixture");
        let original = board.to_fen();
        let mv = board.parse_move(f[3]).expect("legal fixture move");
        let truth: i32 = f[4].parse().unwrap();
        let immediate: i32 = f[5].parse().unwrap();
        let full_expected = if f[1].starts_with("policy-") {
            immediate
        } else {
            truth
        };
        assert_eq!(board.see(mv), full_expected, "{} full SEE", f[0]);
        for threshold in [
            truth - 1,
            truth,
            truth + 1,
            -301,
            -300,
            -299,
            0,
            1,
            immediate,
            immediate + 1,
        ] {
            assert_eq!(
                board.see_ge(mv, threshold),
                full_expected >= threshold,
                "{} threshold {threshold}",
                f[0]
            );
            let aware_expected = if f[1] == "policy-quiet" {
                truth
            } else {
                full_expected
            };
            assert_eq!(
                board.see_ge_quiet_aware(mv, threshold),
                aware_expected >= threshold,
                "{} quiet-aware {threshold}",
                f[0]
            );
        }
        assert_eq!(board.to_fen(), original, "{} changed board", f[0]);
        board
            .check_consistency()
            .expect("SEE must leave keys/state consistent");
    }
    assert_eq!(checked, if selected_debt.is_some() { 1 } else { 15 });
}

#[test]
fn independent_exchange_and_explicit_shortcut_contracts() {
    verify(None);
}

#[test]
#[ignore = "4.11b.5: unresolved exchange defects; remove ignore when repaired"]
fn pending_king_exchange_repair() {
    verify(Some("king-after-pawn"));
}

#[test]
#[ignore = "4.11b.5: pin created during exchange; remove ignore when repaired"]
fn pending_created_pin_repair() {
    verify(Some("pin-created"));
}

#[test]
#[ignore = "4.11b.5: recapture promotion gain; remove ignore when repaired"]
fn pending_recapture_promotion_repair() {
    verify(Some("promotion-recapture"));
}
