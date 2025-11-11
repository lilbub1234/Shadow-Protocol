// Shade Framework - Circuit Optimizer
// Copyright (c) 2025 Shadow Protocol Contributors
// Licensed under MIT License

//! Circuit optimization passes

use crate::core::circuit::*;
use std::collections::{HashMap, HashSet};
use ark_ff::Field as ArkField;

/// Optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub original_constraints: usize,
    pub optimized_constraints: usize,
    pub original_variables: usize,
    pub optimized_variables: usize,
    pub passes_applied: Vec<String>,
    pub reduction_percentage: f64,
}

impl OptimizationStats {
    pub fn new(original_constraints: usize, original_variables: usize) -> Self {
        Self {
            original_constraints,
            optimized_constraints: original_constraints,
            original_variables,
            optimized_variables: original_variables,
            passes_applied: Vec::new(),
            reduction_percentage: 0.0,
        }
    }

    pub fn update(&mut self, new_constraints: usize, new_variables: usize, pass_name: String) {
        self.optimized_constraints = new_constraints;
        self.optimized_variables = new_variables;
        self.passes_applied.push(pass_name);

        self.reduction_percentage = 100.0 * (1.0 - self.optimized_constraints as f64 / self.original_constraints as f64);
    }

    pub fn print_summary(&self) {
        println!("Circuit Optimization Summary:");
        println!("  Original constraints: {}", self.original_constraints);
        println!("  Optimized constraints: {}", self.optimized_constraints);
        println!("  Reduction: {:.1}%", self.reduction_percentage);
        println!("  Passes applied: {}", self.passes_applied.join(", "));
    }
}

/// Main optimizer that applies multiple optimization passes
pub struct CircuitOptimizer {
    pub level: u8, // 0-3, higher = more aggressive
    pub stats: OptimizationStats,
}

impl CircuitOptimizer {
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(3),
            stats: OptimizationStats::new(0, 0),
        }
    }

    /// Optimize a constraint system
    pub fn optimize(&mut self, mut cs: ConstraintSystem) -> Result<ConstraintSystem, CircuitError> {
        self.stats = OptimizationStats::new(cs.constraints.len(), cs.variables.len());

        println!("Starting optimization (level {})...", self.level);

        // Level 1: Basic optimizations
        if self.level >= 1 {
            cs = self.eliminate_dead_constraints(cs)?;
            cs = self.eliminate_dead_variables(cs)?;
        }

        // Level 2: Advanced optimizations
        if self.level >= 2 {
            cs = self.constant_folding(cs)?;
            cs = self.common_subexpression_elimination(cs)?;
        }

        // Level 3: Aggressive optimizations
        if self.level >= 3 {
            cs = self.algebraic_simplification(cs)?;
            cs = self.constraint_merging(cs)?;
        }

        self.stats.update(
            cs.constraints.len(),
            cs.variables.len(),
            "all_passes".to_string(),
        );

        self.stats.print_summary();

        Ok(cs)
    }

    /// Remove constraints that are trivially satisfied
    fn eliminate_dead_constraints(&mut self, mut cs: ConstraintSystem) -> Result<ConstraintSystem, CircuitError> {
        let original_len = cs.constraints.len();

        cs.constraints.retain(|constraint| {
            // Remove constraints like 0 * 1 = 0
            !self.is_trivial_constraint(constraint, &cs)
        });

        let removed = original_len - cs.constraints.len();
        if removed > 0 {
            println!("  Dead constraint elimination: removed {}", removed);
        }

        Ok(cs)
    }

    /// Check if a constraint is trivially satisfied
    fn is_trivial_constraint(&self, constraint: &R1CSConstraint, cs: &ConstraintSystem) -> bool {
        // Check if constraint is 0 * anything = 0
        if constraint.a.terms.is_empty() && constraint.a.constant == Field::from(0) {
            if constraint.c.terms.is_empty() && constraint.c.constant == Field::from(0) {
                return true;
            }
        }

        // Check if constraint is anything * 0 = 0
        if constraint.b.terms.is_empty() && constraint.b.constant == Field::from(0) {
            if constraint.c.terms.is_empty() && constraint.c.constant == Field::from(0) {
                return true;
            }
        }

        // Check if constraint is 1 * 1 = 1
        if constraint.a.terms.is_empty() && constraint.a.constant == Field::from(1) {
            if constraint.b.terms.is_empty() && constraint.b.constant == Field::from(1) {
                if constraint.c.terms.is_empty() && constraint.c.constant == Field::from(1) {
                    return true;
                }
            }
        }

        false
    }

    /// Remove variables that are never used
    fn eliminate_dead_variables(&mut self, mut cs: ConstraintSystem) -> Result<ConstraintSystem, CircuitError> {
        let original_len = cs.variables.len();

        // Find used variables
        let mut used = HashSet::new();
        for constraint in &cs.constraints {
            for (_, var) in &constraint.a.terms {
                used.insert(*var);
            }
            for (_, var) in &constraint.b.terms {
                used.insert(*var);
            }
            for (_, var) in &constraint.c.terms {
                used.insert(*var);
            }
        }

        // Keep only used variables (excluding public/private inputs)
        let num_inputs = cs.num_public + cs.num_private;
        cs.variables.retain(|var| var.0 < num_inputs || used.contains(var));

        let removed = original_len - cs.variables.len();
        if removed > 0 {
            println!("  Dead variable elimination: removed {}", removed);
        }

        Ok(cs)
    }

    /// Fold constants at compile time
    fn constant_folding(&mut self, mut cs: ConstraintSystem) -> Result<ConstraintSystem, CircuitError> {
        let mut changed = false;

        for constraint in &mut cs.constraints {
            // If A and B are constants, compute C = A * B
            if constraint.a.terms.is_empty() && constraint.b.terms.is_empty() {
                let a_val = constraint.a.constant;
                let b_val = constraint.b.constant;
                let c_val = a_val * b_val;

                // Replace C with constant
                if constraint.c.terms.is_empty() {
                    constraint.c.constant = c_val;
                    changed = true;
                }
            }
        }

        if changed {
            println!("  Constant folding: simplified constant expressions");
        }

        Ok(cs)
    }

    /// Eliminate common subexpressions
    fn common_subexpression_elimination(&mut self, mut cs: ConstraintSystem) -> Result<ConstraintSystem, CircuitError> {
        // Map from linear combination hash to variable
        let mut expression_map: HashMap<String, Variable> = HashMap::new();
        let mut replacements: HashMap<Variable, Variable> = HashMap::new();

        // Find duplicate expressions
        for constraint in &cs.constraints {
            for lc in [&constraint.a, &constraint.b, &constraint.c] {
                if lc.terms.len() > 1 {
                    let hash = self.hash_linear_combination(lc);

                    if let Some(&existing_var) = expression_map.get(&hash) {
                        // Found duplicate - mark for replacement
                        if let Some((_, var)) = lc.terms.first() {
                            replacements.insert(*var, existing_var);
                        }
                    } else {
                        // First occurrence - record it
                        if let Some((_, var)) = lc.terms.first() {
                            expression_map.insert(hash, *var);
                        }
                    }
                }
            }
        }

        // Apply replacements
        if !replacements.is_empty() {
            for constraint in &mut cs.constraints {
                self.apply_replacements(&mut constraint.a, &replacements);
                self.apply_replacements(&mut constraint.b, &replacements);
                self.apply_replacements(&mut constraint.c, &replacements);
            }

            println!("  Common subexpression elimination: unified {} expressions", replacements.len());
        }

        Ok(cs)
    }

    /// Simplify algebraic expressions
    fn algebraic_simplification(&mut self, mut cs: ConstraintSystem) -> Result<ConstraintSystem, CircuitError> {
        let mut changed = false;

        for constraint in &mut cs.constraints {
            // Simplify: x * 1 = x
            if constraint.b.terms.is_empty() && constraint.b.constant == Field::from(1) {
                constraint.c = constraint.a.clone();
                changed = true;
            }

            // Simplify: 1 * x = x
            if constraint.a.terms.is_empty() && constraint.a.constant == Field::from(1) {
                constraint.c = constraint.b.clone();
                changed = true;
            }

            // Simplify: x * 0 = 0
            if constraint.b.terms.is_empty() && constraint.b.constant == Field::from(0) {
                constraint.c = LinearCombination::zero();
                changed = true;
            }

            // Simplify: 0 * x = 0
            if constraint.a.terms.is_empty() && constraint.a.constant == Field::from(0) {
                constraint.c = LinearCombination::zero();
                changed = true;
            }
        }

        if changed {
            println!("  Algebraic simplification: simplified algebraic expressions");
        }

        Ok(cs)
    }

    /// Merge compatible constraints
    fn constraint_merging(&mut self, mut cs: ConstraintSystem) -> Result<ConstraintSystem, CircuitError> {
        // This is a complex optimization - simplified version
        // Real implementation would find chains of constraints that can be merged

        let original_len = cs.constraints.len();

        // Remove duplicate constraints
        let mut seen = HashSet::new();
        cs.constraints.retain(|constraint| {
            let hash = format!("{:?}", constraint);
            seen.insert(hash)
        });

        let removed = original_len - cs.constraints.len();
        if removed > 0 {
            println!("  Constraint merging: removed {} duplicate constraints", removed);
        }

        Ok(cs)
    }

    /// Hash a linear combination for CSE
    fn hash_linear_combination(&self, lc: &LinearCombination) -> String {
        let mut s = format!("{:?}", lc.constant);
        for (coeff, var) in &lc.terms {
            s.push_str(&format!("_{:?}_{:?}", coeff, var));
        }
        s
    }

    /// Apply variable replacements to a linear combination
    fn apply_replacements(&self, lc: &mut LinearCombination, replacements: &HashMap<Variable, Variable>) {
        for (_, var) in &mut lc.terms {
            if let Some(&new_var) = replacements.get(var) {
                *var = new_var;
            }
        }
    }
}

/// Quick optimization with default settings
pub fn optimize(cs: ConstraintSystem, level: u8) -> Result<ConstraintSystem, CircuitError> {
    let mut optimizer = CircuitOptimizer::new(level);
    optimizer.optimize(cs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_constraint_elimination() {
        let mut cs = ConstraintSystem::new();

        // Add some real constraints
        let a = cs.alloc_variable(Some(Field::from(2)));
        let b = cs.alloc_variable(Some(Field::from(3)));
        let c = cs.alloc_variable(Some(Field::from(6)));
        cs.enforce_mul(a, b, c);

        // Add a trivial constraint: 0 * 1 = 0
        cs.enforce_constraint(
            LinearCombination::zero(),
            LinearCombination::from_constant(Field::from(1)),
            LinearCombination::zero(),
        );

        assert_eq!(cs.constraints.len(), 2);

        let mut optimizer = CircuitOptimizer::new(1);
        let optimized = optimizer.eliminate_dead_constraints(cs).unwrap();

        assert_eq!(optimized.constraints.len(), 1);
    }

    #[test]
    fn test_constant_folding() {
        let mut cs = ConstraintSystem::new();

        // Constraint: 2 * 3 = c
        let c = cs.alloc_variable(Some(Field::from(6)));
        cs.enforce_constraint(
            LinearCombination::from_constant(Field::from(2)),
            LinearCombination::from_constant(Field::from(3)),
            LinearCombination::from_variable(c),
        );

        let mut optimizer = CircuitOptimizer::new(2);
        let optimized = optimizer.constant_folding(cs).unwrap();

        // After folding, C should be constant 6
        assert_eq!(optimized.constraints[0].c.constant, Field::from(6));
    }

    #[test]
    fn test_full_optimization() {
        let mut cs = ConstraintSystem::new();

        // Create a circuit with redundant constraints
        let a = cs.alloc_variable(Some(Field::from(5)));
        let b = cs.alloc_variable(Some(Field::from(5)));
        let c = cs.alloc_variable(Some(Field::from(25)));

        cs.enforce_mul(a, a, c);
        cs.enforce_mul(b, b, c); // Duplicate

        // Add trivial constraint
        cs.enforce_constraint(
            LinearCombination::zero(),
            LinearCombination::from_constant(Field::from(1)),
            LinearCombination::zero(),
        );

        assert_eq!(cs.constraints.len(), 3);

        let mut optimizer = CircuitOptimizer::new(3);
        let optimized = optimizer.optimize(cs).unwrap();

        // Should have fewer constraints after optimization
        assert!(optimized.constraints.len() < 3);
    }
}
