# ternary-steward

Resource stewardship and **sustainable management** for ternary systems. Provides budget enforcement, audit trails, sustainability metrics, conservation law enforcement, reporting, and intergenerational equity for ternary-valued resources.

## Why It Matters

Systems that consume ternary-classified resources (compute credits, energy budgets, carbon allowances) need three capabilities that traditional accounting lacks:

1. **Classification-aware budgets** — amounts carry ternary classification: `+1` (surplus/credit), `0` (neutral/balanced), `-1` (deficit/debt)
2. **Audit trails with lineage** — every transaction is recorded with source, destination, and classification, enabling post-hoc analysis and compliance checks
3. **Sustainability enforcement** — budgets can enforce that consumption rates stay within regeneration rates, preventing resource exhaustion

This is the conservation-law enforcement layer of the SuperInstance ecosystem.

## How It Works

### Ternary Resource Classification

Every allocation is classified by sign:

```
amount > 0  →  classification = +1  (credit/surplus)
amount = 0  →  classification =  0  (neutral/balanced)
amount < 0  →  classification = -1  (deficit/debt)
```

This classification is automatic and immutable — you don't classify resources manually; the ternary value emerges from the amount itself.

### Budget Model

A `Budget` tracks capacity and current allocation:

```
Budget {
    name: String,
    capacity: i64,
    allocated: i64,
    entries: Vec<Allocation>,
}
```

**Utilization:** `allocated / capacity`

**Conservation invariant:** `|allocated| ≤ capacity` (enforced by the steward)

### Audit Trail

Every transaction is appended to an `AuditTrail`:

```
AuditEntry {
    timestamp: u64,
    description: String,
    entries: Vec<Allocation>,
    balance_before: i64,
    balance_after: i64,
}
```

The audit trail is append-only — entries are never deleted or modified. This provides full lineage for compliance reporting and forensic analysis.

**Complexity:** O(1) append, O(A) replay (A = audit entries).

### Sustainability Metrics

The steward computes sustainability indicators:

| Metric | Formula | Interpretation |
|--------|---------|----------------|
| Utilization | `allocated / capacity` | < 1.0 = sustainable; > 1.0 = over-budget |
| Conservation status | ternary: +1, 0, -1 | +1 = growing surplus; 0 = balanced; -1 = declining |
| Audit health | fraction of valid entries | 1.0 = all transactions compliant |

### Conservation Law Enforcement

The steward enforces conservation by rejecting allocations that would exceed budget capacity:

```
if |allocated + amount| > capacity:
    return Err("budget exceeded")
```

This prevents overcommitment — the conservation law (C) is a hard constraint, not a soft guideline.

**Complexity:** O(1) per allocation check.

## Quick Start

```rust
use ternary_steward::{Steward, Allocation};

let mut steward = Steward::new();
steward.create_budget("compute_credits", capacity: 10_000);

steward.allocate("compute_credits", &Allocation::new("gpu_job_1", 500)).unwrap();
steward.allocate("compute_credits", &Allocation::new("gpu_job_2", 300)).unwrap();

let budget = steward.get_budget("compute_credits").unwrap();
assert_eq!(budget.allocated, 800);
assert_eq!(budget.utilization(), 0.08);
```

## API

| Type | Key Methods |
|------|-------------|
| `Steward` | `create_budget()`, `allocate()`, `get_budget()`, `audit_log()`, `sustainability_report()` |
| `Budget` | `allocate()`, `utilization()`, `balance()`, `entries()` |
| `Allocation` | `new(resource, amount)`, `classification` |
| `AuditTrail` | `record()`, `replay()`, `entry_count()` |
| `Ternary` | `Neg(-1)`, `Zero(0)`, `Pos(+1)` — `from_i8()`, `to_i8()` |

## Architecture Notes

The **γ + η = C** invariant is the foundational principle of this crate. *Generation* (γ) is the allocation process — resources being committed to tasks. *Entropy* (η) is the diversity of resource states across budgets (surplus/deficit spread). *Conservation* (C) is the hard constraint that `Σ allocated ≤ Σ capacity` across all budgets — no resource is created or destroyed, only transferred. The steward enforces C by rejecting over-budget allocations, making the conservation law a runtime invariant rather than a post-hoc audit finding.

## References

- **Resource accounting:** Gray, R. & Bebbington, J. *Accounting for the Environment* (2001)
- **Conservation laws in computing:** Fleischman, G. et al. "A Conservation Law for Computing" (2018)
- **Audit trail design:** Snodgrass, R. *Developing Time-Oriented Database Applications in SQL* (1999)
- **Sustainability metrics:** Atkinson, G. et al. *Measuring Sustainable Development* (2007)

## License

MIT
