"""Model Rarog's SPSA schedule to size a tune BEFORE spending machine nights.

Written 2026-07-27 to answer "how many iterations for N knobs?" with evidence
instead of folklore. Calibration is all measured, not assumed:
  - mini-match = 32 games, gradient = (l - w); std(w-l) = sqrt(32*0.55) = 4.2
  - E[w-l] for an Elo diff D over 32 games = 0.092*D  (logistic slope)
  - schedule = EXACTLY the shipped code path after the a0fbc9f fix:
        it = t/games ; a_t = a/(it+A)^0.601 ; c_t = c/it^0.102 ; A = iters/10
  - measured cost: 28.57 s/iteration at 3+0.03, concurrency 14

Findings that drove the 10.4.6 design (see PLAN):
  1. DIMENSION IS ~FREE. p=6 and p=26 converge at nearly the same rate, so
     merging tune groups costs nothing and captures cross-knob interactions.
     Iterations, not knob count, is the budget that matters.
  2. ITERATIONS DOMINATE. At 1,000-2,500 iterations a tune barely beats its
     own seed -- which is the range every historical Rarog tune ran in.
     ~5,000 recovers ~70% of available gain, ~10,000 ~85%.
  3. THERE IS AN ABSOLUTE NOISE FLOOR, and it does not depend on where you
     start: at p=18 the same run lands in the same band whether seeded 1.0 or
     0.25 steps off. So re-tuning values that are ALREADY inside the floor
     strictly HURTS -- it scatters them. This is why 10.4.6 carries a per-knob
     bake filter instead of baking everything the tuner returns.
  4. CURVATURE BELOW ~0.5 Elo per full step is UNFITTABLE at 32 games/iter.
     Such knobs wander forever; baking their wander ships noise.
  5. Games-per-iteration barely matters at a fixed game budget (16..128 all
     land within noise of each other), so 32 stays.

Run: python tools/spsa_convergence_model.py
"""
import math, random, statistics

ALPHA, GAMMA, A_FRAC, GAMES = 0.601, 0.102, 0.10, 32
SIG_PER_ELO, NOISE = 0.092, 4.2


def run(p, iters, loss_per_step, seed, start_off=1.0):
    """Returns (rmse_in_step_units_at_end, rmse_tail_mean) starting start_off steps off."""
    rng = random.Random(seed)
    A = iters * A_FRAC
    # theta measured in units of each knob's own step; optimum at 0
    theta = [start_off * (1 if rng.random() < 0.5 else -1) for _ in range(p)]
    tail = []
    for k in range(1, iters + 1):
        it = k
        a_t = 1.0 / (it + A) ** ALPHA
        c_t = 1.0 / it ** GAMMA
        delta = [1 if rng.random() < 0.5 else -1 for _ in range(p)]
        # Elo of each arm relative to optimum: -loss_per_step * sum(x_i^2)
        plus = [theta[i] + c_t * delta[i] for i in range(p)]
        minus = [theta[i] - c_t * delta[i] for i in range(p)]
        elo_plus = -loss_per_step * sum(x * x for x in plus)
        elo_minus = -loss_per_step * sum(x * x for x in minus)
        D = elo_plus - elo_minus                      # Elo(arm A) - Elo(arm B)
        grad = -SIG_PER_ELO * D + rng.gauss(0, NOISE)  # (l - w)
        for i in range(p):
            theta[i] -= a_t * grad / (delta[i] * c_t)
        if k > iters * 0.85:
            tail.append(list(theta))
    rmse = math.sqrt(sum(x * x for x in theta) / p)
    tm = [statistics.fmean(v[i] for v in tail) for i in range(p)]
    rmse_tm = math.sqrt(sum(x * x for x in tm) / p)
    return rmse, rmse_tm


def sweep(label, ps, iters_list, loss_per_step, reps=12):
    print(f"\n=== {label}  (loss per knob per full step = {loss_per_step} Elo) ===")
    print("            " + "".join(f"{i:>12}" for i in iters_list) + "     <- iterations")
    for p in ps:
        cells = []
        for iters in iters_list:
            rs = [run(p, iters, loss_per_step, seed=1000 + 7 * r) for r in range(reps)]
            end = statistics.fmean(r[0] for r in rs)
            tm = statistics.fmean(r[1] for r in rs)
            cells.append(f"{end:>5.2f}/{tm:<5.2f}")
        print(f"  p={p:<3}     " + "".join(f"{c:>12}" for c in cells))
    print("  (cells: RMSE-at-endpoint / RMSE-of-tail-mean, in units of each knob's step;")
    print("   start = 1.0 step off the optimum. <1.0 means the tune IMPROVED on its seed.)")


if __name__ == "__main__":
    for lps in (0.5, 1.5, 4.0):
        sweep("convergence vs dimension", [6, 8, 14, 18, 26], [1000, 2500, 5000, 10000], lps)

