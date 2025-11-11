// Shade Framework - Circuit Module
// Copyright (c) 2025 Shadow Protocol Contributors
// Licensed under MIT License

//! Core circuit definitions and building blocks

use std::fmt;
use std::collections::HashMap;

pub mod gadgets;
pub mod primitives;
pub mod witness;

/// Unique identifier for circuits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CircuitId(u64);

impl CircuitId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        CircuitId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// Field element (using BN254 curve by default)
pub type Field = ark_bn254::Fr;

/// Variable in a circuit (represents a wire)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable(pub usize);

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// Linear combination of variables
#[derive(Debug, Clone)]
pub struct LinearCombination {
    pub terms: Vec<(Field, Variable)>,
    pub constant: Field,
}

impl LinearCombination {
    pub fn zero() -> Self {
        Self {
            terms: Vec::new(),
            constant: Field::from(0),
        }
    }

    pub fn from_variable(var: Variable) -> Self {
        Self {
            terms: vec![(Field::from(1), var)],
            constant: Field::from(0),
        }
    }

    pub fn from_constant(c: Field) -> Self {
        Self {
            terms: Vec::new(),
            constant: c,
        }
    }

    /// Add another linear combination
    pub fn add(&mut self, other: &LinearCombination) {
        self.terms.extend_from_slice(&other.terms);
        self.constant += other.constant;
    }

    /// Scale by a constant
    pub fn scale(&mut self, scalar: Field) {
        for (coeff, _) in &mut self.terms {
            *coeff *= scalar;
        }
        self.constant *= scalar;
    }
}

/// Core circuit trait - implement this to define a circuit
pub trait Circuit {
    /// Build the constraints for this circuit
    fn build_constraints(&self, cs: &mut ConstraintSystem) -> Result<(), CircuitError>;

    /// Get number of public inputs
    fn num_public_inputs(&self) -> usize;

    /// Get number of private inputs
    fn num_private_inputs(&self) -> usize;

    /// Get circuit identifier
    fn id(&self) -> CircuitId;
}

/// Constraint system builder
pub struct ConstraintSystem {
    /// All variables in the circuit
    pub variables: Vec<Variable>,

    /// Variable assignments (for witness generation)
    pub assignments: HashMap<Variable, Field>,

    /// R1CS constraints: A * B = C
    pub constraints: Vec<R1CSConstraint>,

    /// Number of public inputs
    pub num_public: usize,

    /// Number of private inputs (witness)
    pub num_private: usize,

    /// Next variable index
    next_var: usize,
}

impl ConstraintSystem {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            assignments: HashMap::new(),
            constraints: Vec::new(),
            num_public: 0,
            num_private: 0,
            next_var: 0,
        }
    }

    /// Allocate a new variable
    pub fn alloc_variable(&mut self, value: Option<Field>) -> Variable {
        let var = Variable(self.next_var);
        self.next_var += 1;
        self.variables.push(var);

        if let Some(v) = value {
            self.assignments.insert(var, v);
        }

        var
    }

    /// Allocate a public input variable
    pub fn alloc_public_input(&mut self, value: Field) -> Variable {
        self.num_public += 1;
        self.alloc_variable(Some(value))
    }

    /// Allocate a private input variable (witness)
    pub fn alloc_private_input(&mut self, value: Field) -> Variable {
        self.num_private += 1;
        self.alloc_variable(Some(value))
    }

    /// Add a constraint: A * B = C
    pub fn enforce_constraint(
        &mut self,
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
    ) {
        self.constraints.push(R1CSConstraint { a, b, c });
    }

    /// Enforce that two linear combinations are equal
    pub fn enforce_equal(&mut self, a: LinearCombination, b: LinearCombination) {
        // a = b  =>  (a - b) * 1 = 0
        let mut diff = a.clone();
        let mut neg_b = b.clone();
        neg_b.scale(Field::from(-1));
        diff.add(&neg_b);

        let one = LinearCombination::from_constant(Field::from(1));
        let zero = LinearCombination::zero();

        self.enforce_constraint(diff, one, zero);
    }

    /// Enforce multiplication: a * b = c
    pub fn enforce_mul(
        &mut self,
        a: Variable,
        b: Variable,
        c: Variable,
    ) {
        self.enforce_constraint(
            LinearCombination::from_variable(a),
            LinearCombination::from_variable(b),
            LinearCombination::from_variable(c),
        );
    }

    /// Get variable assignment
    pub fn get_value(&self, var: Variable) -> Option<Field> {
        self.assignments.get(&var).copied()
    }

    /// Set variable assignment
    pub fn set_value(&mut self, var: Variable, value: Field) {
        self.assignments.insert(var, value);
    }

    /// Check if all constraints are satisfied
    pub fn is_satisfied(&self) -> bool {
        use ark_ff::Field as ArkField;

        for constraint in &self.constraints {
            // Evaluate A
            let mut a_val = constraint.a.constant;
            for (coeff, var) in &constraint.a.terms {
                if let Some(val) = self.assignments.get(var) {
                    a_val += *coeff * val;
                } else {
                    return false; // Missing assignment
                }
            }

            // Evaluate B
            let mut b_val = constraint.b.constant;
            for (coeff, var) in &constraint.b.terms {
                if let Some(val) = self.assignments.get(var) {
                    b_val += *coeff * val;
                } else {
                    return false;
                }
            }

            // Evaluate C
            let mut c_val = constraint.c.constant;
            for (coeff, var) in &constraint.c.terms {
                if let Some(val) = self.assignments.get(var) {
                    c_val += *coeff * val;
                } else {
                    return false;
                }
            }

            // Check A * B = C
            if a_val * b_val != c_val {
                return false;
            }
        }

        true
    }

    /// Get constraint count
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }
}

/// R1CS constraint: A * B = C
#[derive(Debug, Clone)]
pub struct R1CSConstraint {
    pub a: LinearCombination,
    pub b: LinearCombination,
    pub c: LinearCombination,
}

/// Circuit errors
#[derive(Debug)]
pub enum CircuitError {
    InvalidWitness(String),
    ConstraintNotSatisfied(String),
    InvalidInput(String),
    CompilationError(String),
}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitError::InvalidWitness(msg) => write!(f, "Invalid witness: {}", msg),
            CircuitError::ConstraintNotSatisfied(msg) => {
                write!(f, "Constraint not satisfied: {}", msg)
            }
            CircuitError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CircuitError::CompilationError(msg) => write!(f, "Compilation error: {}", msg),
        }
    }
}

impl std::error::Error for CircuitError {}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Field as ArkField;

    #[test]
    fn test_constraint_system_simple() {
        let mut cs = ConstraintSystem::new();

        // Circuit: x * x = y
        let x = cs.alloc_variable(Some(Field::from(3)));
        let y = cs.alloc_variable(Some(Field::from(9)));

        cs.enforce_mul(x, x, y);

        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_constraint_system_addition() {
        let mut cs = ConstraintSystem::new();

        // Circuit: (x + y) * 1 = z
        let x = cs.alloc_variable(Some(Field::from(5)));
        let y = cs.alloc_variable(Some(Field::from(7)));
        let z = cs.alloc_variable(Some(Field::from(12)));

        let mut lc_sum = LinearCombination::from_variable(x);
        lc_sum.add(&LinearCombination::from_variable(y));

        cs.enforce_constraint(
            lc_sum,
            LinearCombination::from_constant(Field::from(1)),
            LinearCombination::from_variable(z),
        );

        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_constraint_not_satisfied() {
        let mut cs = ConstraintSystem::new();

        // Circuit: x * x = y (but y is wrong)
        let x = cs.alloc_variable(Some(Field::from(3)));
        let y = cs.alloc_variable(Some(Field::from(10))); // Should be 9

        cs.enforce_mul(x, x, y);

        assert!(!cs.is_satisfied());
    }

    #[test]
    fn test_linear_combination() {
        let mut lc = LinearCombination::from_variable(Variable(0));
        lc.add(&LinearCombination::from_constant(Field::from(5)));
        lc.scale(Field::from(2));

        assert_eq!(lc.constant, Field::from(10));
        assert_eq!(lc.terms.len(), 1);
        assert_eq!(lc.terms[0].0, Field::from(2));
    }
}
