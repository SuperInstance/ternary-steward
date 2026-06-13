# Ternary Steward — Resource Stewardship and Conservation for Ternary Systems

**Ternary Steward** implements sustainable resource management for ternary systems with budget enforcement, audit trails, sustainability measurement, and intergenerational equity. It is the conservation law enforcement layer — ensuring that resource consumption stays within declared budgets and that all allocations are classified ternarily as {positive (growth), neutral (balanced), negative (depletion)}.

## Why It Matters

Every system has finite resources. Without explicit budget tracking, consumption grows until the system collapses. The steward pattern solves this by maintaining named budgets with hard limits, classifying every allocation as growth (+1), balanced (0), or depletion (-1), and maintaining an immutable audit trail. This is especially important for ternary fleets where GPU time, memory, and bandwidth are precious: the steward ensures that growth operations (γ) don't starve maintenance operations (η), and that the total stays within capacity C.

## How It Works

### Budgets

Each `Budget` has a name, capacity (maximum amount), and current utilization. Allocations that would exceed capacity are rejected. Budget tracking is O(1) per allocation.

### Allocations

An `Allocation` records a resource, amount, and ternary classification:
- **Positive (+1)**: Amount > 0 — resource is being added (growth investment)
- **Zero (0)**: Amount = 0 — neutral, balanced transaction
- **Negative (-1)**: Amount < 0 — resource is being consumed (depletion)

### Audit Trail

Every allocation is recorded in the `AuditTrail` with timestamp, resource, amount, and classification. The trail is append-only and supports querying by resource, time range, or classification. O(1) append, O(n) query.

### Sustainability Metrics

The steward computes:
- **Utilization rate**: current_usage / capacity — should stay below 80% for headroom
- **Depletion rate**: negative_allocations / total_allocations — should stay below 30%
- **Growth index**: positive_allocations / total_allocations — measure of fleet investment
- **Balance score**: How close the ternary distribution is to uniform (healthy fleet)

### Intergenerational Equity

The steward enforces fairness across time periods: no single period can consume more than a configurable fraction of remaining budget. This ensures future generations of agents have resources.

## Quick Start

```rust
use ternary_steward::{Steward, Allocation};

let mut steward = Steward::new();
steward.create_budget("gpu_hours", 10000);
steward.create_budget("memory_gb", 500);

// Make allocations
let alloc = Allocation::new("gpu_hours", 100);
// steward.allocate(alloc)?;

// Check utilization
// let budget = steward.get_budget("gpu_hours").unwrap();
// println!("Utilization: {:.1}%", budget.utilization() * 100.0);
```

```bash
cargo add ternary-steward
```

## API

| Type / Function | Description |
|---|---|
| `Steward` | Central manager: `create_budget()`, `allocate()`, audit queries |
| `Budget` | `{ name, capacity, current }` |
| `Allocation` | `{ resource, amount, classification: Ternary }` |
| `AuditTrail` | Append-only log of all allocations |
| `Ternary` | `Neg(-1)`, `Zero(0)`, `Pos(1)` |

## Architecture Notes

The steward is the conservation enforcement layer in **SuperInstance**. It directly enforces the γ + η = C conservation law: γ allocations (positive) and η allocations (negative) are tracked against budget C. When γ + η would exceed C, the steward rejects the allocation. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Solow, Robert. "An Almost Practical Step toward Sustainability," *Resources for the Future*, 1992 — intergenerational equity.
| Hardin, Garrett. "The Tragedy of the Commons," *Science*, 162(3859), 1968.
| Ostrom, Elinor. *Governing the Commons*, Cambridge UP, 1990 — resource stewardship.



## Complexity Summary

| Operation | Time | Notes |
|---|---|---|
| Budget creation | O(1) | HashMap insert |
| Allocation | O(1) | Amount + classification |
| Audit query (by resource) | O(n) | Filter audit entries |
| Audit query (by time range) | O(n) | Linear scan |
| Sustainability computation | O(b) for b budgets | Aggregate statistics |

The steward adds O(1) overhead per resource transaction, enabling real-time budget enforcement without measurable performance impact on fleet operations.

## License

MIT
