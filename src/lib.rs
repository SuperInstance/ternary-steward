#![forbid(unsafe_code)]

//! Resource stewardship and sustainable management for ternary systems.
//!
//! Provides budgeting, auditing, sustainability measurement, conservation
//! enforcement, reporting, and intergenerational equity for ternary resources.
//! Maps to the conservation law enforcement layer in the SuperInstance ecosystem.

use std::collections::HashMap;

/// Ternary value: -1, 0, or +1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Ternary {
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    pub fn to_i8(self) -> i8 {
        self as i8
    }
}

/// A named resource with a ternary state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResourceId(pub &'static str);

/// A resource allocation entry.
#[derive(Clone, Debug, PartialEq)]
pub struct Allocation {
    pub resource: String,
    pub amount: i64,
    pub classification: Ternary,
}

impl Allocation {
    pub fn new(resource: &str, amount: i64) -> Self {
        let classification = if amount > 0 {
            Ternary::Pos
        } else if amount < 0 {
            Ternary::Neg
        } else {
            Ternary::Zero
        };
        Allocation {
            resource: resource.to_string(),
            amount,
            classification,
        }
    }
}

/// The steward: central resource manager.
#[derive(Debug)]
pub struct Steward {
    budgets: HashMap<String, Budget>,
    audit: AuditTrail,
}

impl Steward {
    pub fn new() -> Self {
        Steward {
            budgets: HashMap::new(),
            audit: AuditTrail::new(),
        }
    }

    pub fn create_budget(&mut self, name: &str, capacity: i64) -> &mut Budget {
        let budget = Budget::new(name, capacity);
        self.budgets.insert(name.to_string(), budget);
        self.audit.record("steward", &format!("Created budget '{}' with capacity {}", name, capacity));
        self.budgets.get_mut(name).unwrap()
    }

    pub fn budget(&self, name: &str) -> Option<&Budget> {
        self.budgets.get(name)
    }

    pub fn budget_mut(&mut self, name: &str) -> Option<&mut Budget> {
        self.budgets.get_mut(name)
    }

    pub fn audit(&self) -> &AuditTrail {
        &self.audit
    }

    pub fn sustainability_index(&self) -> SustainabilityIndex {
        let mut total_utilization = 0.0_f64;
        let mut count = 0;
        for budget in self.budgets.values() {
            total_utilization += budget.utilization();
            count += 1;
        }
        let avg = if count > 0 { total_utilization / count as f64 } else { 0.0 };
        SustainabilityIndex::from_utilization(avg)
    }

    pub fn budget_count(&self) -> usize {
        self.budgets.len()
    }
}

impl Default for Steward {
    fn default() -> Self {
        Self::new()
    }
}

/// Allocates ternary resources within capacity constraints.
#[derive(Clone, Debug)]
pub struct Budget {
    pub name: String,
    pub capacity: i64,
    allocated: i64,
    spent: i64,
}

impl Budget {
    pub fn new(name: &str, capacity: i64) -> Self {
        Budget {
            name: name.to_string(),
            capacity: capacity.max(0),
            allocated: 0,
            spent: 0,
        }
    }

    /// Allocate resources. Returns true if within capacity.
    pub fn allocate(&mut self, amount: i64) -> bool {
        let new_total = self.allocated + amount;
        if new_total <= self.capacity && new_total >= 0 {
            self.allocated = new_total;
            true
        } else {
            false
        }
    }

    /// Spend from allocated resources.
    pub fn spend(&mut self, amount: i64) -> bool {
        if amount <= self.available() {
            self.spent += amount;
            true
        } else {
            false
        }
    }

    pub fn available(&self) -> i64 {
        self.allocated - self.spent
    }

    pub fn remaining_capacity(&self) -> i64 {
        self.capacity - self.allocated
    }

    pub fn utilization(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.allocated as f64 / self.capacity as f64
    }

    pub fn is_exhausted(&self) -> bool {
        self.allocated >= self.capacity
    }
}

/// Tracks all resource changes over time.
#[derive(Clone, Debug)]
pub struct AuditEntry {
    pub sequence: u64,
    pub source: String,
    pub action: String,
}

/// Immutable log of all resource changes.
#[derive(Debug, Clone)]
pub struct AuditTrail {
    entries: Vec<AuditEntry>,
    next_seq: u64,
}

impl AuditTrail {
    pub fn new() -> Self {
        AuditTrail {
            entries: Vec::new(),
            next_seq: 0,
        }
    }

    pub fn record(&mut self, source: &str, action: &str) -> AuditEntry {
        let entry = AuditEntry {
            sequence: self.next_seq,
            source: source.to_string(),
            action: action.to_string(),
        };
        self.next_seq += 1;
        self.entries.push(entry.clone());
        entry
    }

    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Filter entries by source.
    pub fn by_source(&self, source: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.source == source).collect()
    }

    /// Filter entries containing keyword.
    pub fn search(&self, keyword: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.action.contains(keyword)).collect()
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

/// Measures long-term viability of resource usage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SustainabilityIndex {
    pub score: f64, // 0.0 to 1.0
}

impl SustainabilityIndex {
    pub fn from_utilization(avg_utilization: f64) -> Self {
        // Optimal utilization around 0.5-0.7; penalize extremes
        let score = if avg_utilization <= 0.0 {
            1.0 // nothing used = fully sustainable
        } else if avg_utilization <= 0.7 {
            1.0 - avg_utilization * 0.3 // gentle decline: 1.0 → 0.79
        } else if avg_utilization <= 0.9 {
            0.79 - (avg_utilization - 0.7) * 2.0 // warning zone: 0.79 → 0.39
        } else {
            0.39 - (avg_utilization - 0.9) * 2.5 // danger zone: 0.39 → ~0.0
        };
        SustainabilityIndex {
            score: score.clamp(0.0, 1.0),
        }
    }

    pub fn is_sustainable(&self) -> bool {
        self.score >= 0.5
    }

    pub fn status(&self) -> &'static str {
        if self.score >= 0.8 {
            "healthy"
        } else if self.score >= 0.5 {
            "moderate"
        } else if self.score >= 0.2 {
            "strained"
        } else {
            "critical"
        }
    }
}

/// Enforces conservation laws on resource usage.
#[derive(Debug)]
pub struct ConservationOfficer {
    pub max_utilization: f64,
    pub warnings_issued: u32,
}

impl ConservationOfficer {
    pub fn new(max_utilization: f64) -> Self {
        ConservationOfficer {
            max_utilization: max_utilization.clamp(0.0, 1.0),
            warnings_issued: 0,
        }
    }

    /// Check if a budget complies with conservation laws.
    pub fn check(&mut self, budget: &Budget) -> ConservationVerdict {
        let util = budget.utilization();
        if util > self.max_utilization {
            self.warnings_issued += 1;
            ConservationVerdict::Violation {
                budget: budget.name.clone(),
                utilization: util,
                limit: self.max_utilization,
            }
        } else if util > self.max_utilization * 0.9 {
            ConservationVerdict::Warning {
                budget: budget.name.clone(),
                utilization: util,
            }
        } else {
            ConservationVerdict::Compliant
        }
    }

    /// Enforce: return the budget's allowed allocation amount.
    pub fn enforce_limit(&self, budget: &Budget, requested: i64) -> i64 {
        let remaining = (budget.capacity as f64 * self.max_utilization) as i64 - budget.allocated;
        requested.min(remaining.max(0))
    }
}

/// Result of a conservation check.
#[derive(Clone, Debug, PartialEq)]
pub enum ConservationVerdict {
    Compliant,
    Warning { budget: String, utilization: f64 },
    Violation { budget: String, utilization: f64, limit: f64 },
}

/// Generates structured reports on resource state.
#[derive(Debug)]
pub struct StewardReport;

impl StewardReport {
    pub fn generate(steward: &Steward) -> Report {
        let mut budget_reports = Vec::new();
        let mut total_capacity = 0i64;
        let mut total_allocated = 0i64;

        for (name, budget) in &steward.budgets {
            budget_reports.push(BudgetReport {
                name: name.clone(),
                capacity: budget.capacity,
                allocated: budget.allocated,
                spent: budget.spent,
                utilization: budget.utilization(),
            });
            total_capacity += budget.capacity;
            total_allocated += budget.allocated;
        }

        let overall_utilization = if total_capacity > 0 {
            total_allocated as f64 / total_capacity as f64
        } else {
            0.0
        };

        Report {
            total_budgets: budget_reports.len(),
            total_capacity,
            total_allocated,
            overall_utilization,
            sustainability: steward.sustainability_index(),
            audit_entries: steward.audit().len(),
            budgets: budget_reports,
        }
    }
}

/// A single budget's report.
#[derive(Clone, Debug, PartialEq)]
pub struct BudgetReport {
    pub name: String,
    pub capacity: i64,
    pub allocated: i64,
    pub spent: i64,
    pub utilization: f64,
}

/// Full stewardship report.
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub total_budgets: usize,
    pub total_capacity: i64,
    pub total_allocated: i64,
    pub overall_utilization: f64,
    pub sustainability: SustainabilityIndex,
    pub audit_entries: usize,
    pub budgets: Vec<BudgetReport>,
}

/// Ensures future agents have sufficient resources (intergenerational equity).
#[derive(Debug)]
pub struct IntergenerationalEquity {
    pub reserve_ratio: f64, // fraction of resources to reserve
    pub generation: u64,
}

impl IntergenerationalEquity {
    pub fn new(reserve_ratio: f64) -> Self {
        IntergenerationalEquity {
            reserve_ratio: reserve_ratio.clamp(0.0, 1.0),
            generation: 0,
        }
    }

    /// Calculate how much of a resource can be used now.
    pub fn allowable_usage(&self, total: i64) -> i64 {
        let usable = total as f64 * (1.0 - self.reserve_ratio);
        usable as i64
    }

    /// Calculate the reserve amount.
    pub fn reserve(&self, total: i64) -> i64 {
        (total as f64 * self.reserve_ratio) as i64
    }

    /// Advance to the next generation.
    pub fn next_generation(&mut self) -> u64 {
        self.generation += 1;
        self.generation
    }

    /// Check if a proposed usage violates intergenerational equity.
    pub fn is_equitable(&self, total: i64, proposed_usage: i64) -> bool {
        proposed_usage <= self.allowable_usage(total)
    }

    /// Evaluate equity across multiple budgets.
    pub fn evaluate(&self, budgets: &[&Budget]) -> EquityReport {
        let mut violations = Vec::new();
        let mut total_reserve = 0i64;

        for budget in budgets {
            let reserve = self.reserve(budget.capacity);
            total_reserve += reserve;
            if budget.allocated > self.allowable_usage(budget.capacity) {
                violations.push(budget.name.clone());
            }
        }

        EquityReport {
            generation: self.generation,
            reserve_ratio: self.reserve_ratio,
            total_reserve,
            violation_count: violations.len(),
            violations,
        }
    }
}

/// Report on intergenerational equity.
#[derive(Clone, Debug, PartialEq)]
pub struct EquityReport {
    pub generation: u64,
    pub reserve_ratio: f64,
    pub total_reserve: i64,
    pub violation_count: usize,
    pub violations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_allocate() {
        let mut b = Budget::new("cpu", 100);
        assert!(b.allocate(50));
        assert_eq!(b.allocated, 50);
        assert!(b.allocate(50));
        assert_eq!(b.allocated, 100);
        assert!(!b.allocate(1)); // over capacity
    }

    #[test]
    fn test_budget_spend() {
        let mut b = Budget::new("mem", 100);
        b.allocate(60);
        assert!(b.spend(30));
        assert_eq!(b.spent, 30);
        assert_eq!(b.available(), 30);
        assert!(!b.spend(40)); // not enough available
    }

    #[test]
    fn test_budget_utilization() {
        let mut b = Budget::new("disk", 200);
        b.allocate(100);
        assert!((b.utilization() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_budget_exhausted() {
        let mut b = Budget::new("net", 10);
        b.allocate(10);
        assert!(b.is_exhausted());
    }

    #[test]
    fn test_budget_remaining_capacity() {
        let mut b = Budget::new("gpu", 100);
        b.allocate(30);
        assert_eq!(b.remaining_capacity(), 70);
    }

    #[test]
    fn test_steward_create_and_access_budget() {
        let mut s = Steward::new();
        s.create_budget("cpu", 100);
        assert!(s.budget("cpu").is_some());
        assert_eq!(s.budget_count(), 1);
    }

    #[test]
    fn test_steward_sustainability() {
        let mut s = Steward::new();
        s.create_budget("cpu", 100);
        let idx = s.sustainability_index();
        assert!(idx.is_sustainable()); // 0% utilization = sustainable
    }

    #[test]
    fn test_audit_trail_record() {
        let mut a = AuditTrail::new();
        a.record("test", "created budget");
        a.record("test", "allocated 50");
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn test_audit_trail_by_source() {
        let mut a = AuditTrail::new();
        a.record("alpha", "action 1");
        a.record("beta", "action 2");
        a.record("alpha", "action 3");
        assert_eq!(a.by_source("alpha").len(), 2);
    }

    #[test]
    fn test_audit_trail_search() {
        let mut a = AuditTrail::new();
        a.record("src", "allocated 50 to cpu");
        a.record("src", "spent 20 from mem");
        let results = a.search("cpu");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_sustainability_healthy() {
        let idx = SustainabilityIndex::from_utilization(0.1);
        assert_eq!(idx.status(), "healthy");
        assert!(idx.is_sustainable());
    }

    #[test]
    fn test_sustainability_strained() {
        let idx = SustainabilityIndex::from_utilization(0.85);
        assert_eq!(idx.status(), "strained");
        assert!(!idx.is_sustainable());
    }

    #[test]
    fn test_sustainability_critical() {
        let idx = SustainabilityIndex::from_utilization(0.99);
        assert_eq!(idx.status(), "critical");
    }

    #[test]
    fn test_conservation_officer_compliant() {
        let mut co = ConservationOfficer::new(0.8);
        let b = Budget::new("cpu", 100);
        assert_eq!(co.check(&b), ConservationVerdict::Compliant);
    }

    #[test]
    fn test_conservation_officer_violation() {
        let mut co = ConservationOfficer::new(0.5);
        let mut b = Budget::new("cpu", 100);
        b.allocate(80);
        let result = co.check(&b);
        assert!(matches!(result, ConservationVerdict::Violation { .. }));
        assert_eq!(co.warnings_issued, 1);
    }

    #[test]
    fn test_conservation_officer_enforce_limit() {
        let co = ConservationOfficer::new(0.5);
        let b = Budget::new("cpu", 100);
        let allowed = co.enforce_limit(&b, 60);
        assert_eq!(allowed, 50); // 50% of 100
    }

    #[test]
    fn test_steward_report() {
        let mut s = Steward::new();
        s.create_budget("cpu", 100);
        s.budget_mut("cpu").unwrap().allocate(50);
        let report = StewardReport::generate(&s);
        assert_eq!(report.total_budgets, 1);
        assert!((report.overall_utilization - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_intergenerational_equity_allowable() {
        let eq = IntergenerationalEquity::new(0.2);
        assert_eq!(eq.allowable_usage(100), 80);
        assert_eq!(eq.reserve(100), 20);
    }

    #[test]
    fn test_intergenerational_equity_is_equitable() {
        let eq = IntergenerationalEquity::new(0.3);
        assert!(eq.is_equitable(100, 70));
        assert!(!eq.is_equitable(100, 71));
    }

    #[test]
    fn test_intergenerational_equity_evaluate() {
        let eq = IntergenerationalEquity::new(0.2);
        let mut b = Budget::new("cpu", 100);
        b.allocate(90);
        let report = eq.evaluate(&[&b]);
        assert_eq!(report.violation_count, 1);
        assert!(report.violations.contains(&"cpu".to_string()));
    }

    #[test]
    fn test_intergenerational_next_generation() {
        let mut eq = IntergenerationalEquity::new(0.1);
        assert_eq!(eq.next_generation(), 1);
        assert_eq!(eq.next_generation(), 2);
    }
}
