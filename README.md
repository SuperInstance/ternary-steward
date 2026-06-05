# ternary-steward

**Resource stewardship and sustainability for ternary systems. Budgets, audits, conservation.**

A steward doesn't own the resources — they manage them for the future. This crate implements sustainable resource management for ternary systems: budgeting (allocate resources across categories), auditing (verify compliance with budgets), sustainability measurement (are we consuming faster than we're replenishing?), and intergenerational equity (are we leaving enough for the next generation?).

Every resource has a ternary state: `-1` (depleted/below threshold), `0` (adequate/at target), `+1` (abundant/above target). The steward's job is to keep everything at 0 — not too much, not too little.

## What's Inside

- **`Steward`** — manages named resources with ternary states, budgets, and history
- **`Budget`** — allocation per category with min/max thresholds
- **`audit(steward, budget)`** — verify compliance: any resource outside thresholds triggers a finding
- **`sustainability_score(history)`** — are resources trending up (1), stable (0), or down (-1)?
- **`conservation_enforcement(state, budget)`** — clamp consumption to budget limits
- **`intergenerational_equity(current, reserves, horizon)`** — can we sustain current consumption for N generations?

## Quick Example

```rust
use ternary_steward::*;

let mut steward = Steward::new();
steward.add_resource("energy", 0);    // adequate
steward.add_resource("water", 1);     // abundant
steward.add_resource("compute", -1);  // depleted

let budget = Budget::new()
    .allocate("energy", -0.5, 0.5)   // keep near 0
    .allocate("water", 0.0, 1.0)     // at least adequate
    .allocate("compute", -1.0, 0.5); // allow some depletion

let findings = audit(&steward, &budget);
// "compute" at -1 is within range, but trending down

let score = sustainability_score(&steward.history("energy"));
println!("Energy sustainability: {:?}", score);
```

## The Deeper Truth

**Zero is the target, not the minimum.** In most systems, "zero" means "nothing" — a failure state. In stewardship, zero means *balanced* — the resource is exactly where it should be. Too much (+1) is as much a problem as too little (-1): abundance can lead to waste, inflation, or dependency. The steward aims for the spindle — the balanced center where resources flow sustainably.

The intergenerational equity function is the most important: given current consumption and known reserves, can this continue for N generations? If not, the steward must either reduce consumption or find new reserves. The ternary output tells you: +1 = sustainable for the horizon, 0 = borderline, -1 = will run out.

**Use cases:**
- **Cloud infrastructure** — manage compute, memory, network budgets
- **Environmental modeling** — resource sustainability with ternary health indicators
- **Financial planning** — budget stewardship across categories
- **Multi-agent resource allocation** — fair distribution with conservation constraints
- **Game design** — resource management mechanics (RTS, survival games)

## See Also

- **ternary-budget** — (if it exists) focused budget management
- **ternary-cell** — 3-byte cells that track resource state at scale
- **ternary-grace** — the cost of entering the depleted state
- **ternary-experiment** — sweep sustainability parameters

## Install

```bash
cargo add ternary-steward
```

## License

MIT
