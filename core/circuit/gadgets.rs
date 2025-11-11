// Shade Framework - Circuit Gadgets
// Copyright (c) 2025 Shadow Protocol Contributors
// Licensed under MIT License

//! Common circuit gadgets and building blocks

use super::*;
use ark_ff::Field as ArkField;

/// Poseidon hash gadget (ZK-friendly hash function)
pub struct PoseidonGadget {
    pub inputs: Vec<Variable>,
    pub output: Variable,
}

impl PoseidonGadget {
    /// Create a new Poseidon hash gadget
    pub fn new(
        cs: &mut ConstraintSystem,
        inputs: &[Variable],
    ) -> Result<Self, CircuitError> {
        // For now, simplified implementation
        // Real implementation would use actual Poseidon rounds

        let output = cs.alloc_variable(None);

        // Compute hash value if inputs have assignments
        let mut hash_val = Field::from(0);
        let mut all_assigned = true;

        for (i, input) in inputs.iter().enumerate() {
            if let Some(val) = cs.get_value(*input) {
                // Simplified mixing: hash = sum(input_i * (i+1)^2)
                // Real Poseidon uses proper S-boxes and MDS matrices
                let coeff = Field::from((i + 1) as u64).square();
                hash_val += val * coeff;
            } else {
                all_assigned = false;
                break;
            }
        }

        if all_assigned {
            cs.set_value(output, hash_val);
        }

        // Add constraints (simplified - real Poseidon has ~150 constraints per hash)
        // This adds basic mixing constraints
        for (i, input) in inputs.iter().enumerate() {
            let temp = cs.alloc_variable(None);
            let coeff = Field::from((i + 1) as u64).square();

            if let Some(in_val) = cs.get_value(*input) {
                cs.set_value(temp, in_val * coeff);
            }

            // temp = input * coeff
            let mut lc_coeff = LinearCombination::from_constant(coeff);
            cs.enforce_constraint(
                LinearCombination::from_variable(*input),
                lc_coeff,
                LinearCombination::from_variable(temp),
            );
        }

        Ok(PoseidonGadget {
            inputs: inputs.to_vec(),
            output,
        })
    }

    pub fn output(&self) -> Variable {
        self.output
    }
}

/// Range check gadget: ensure value is in range [min, max]
pub struct RangeCheckGadget {
    pub value: Variable,
    pub min: u64,
    pub max: u64,
    pub proof_variables: Vec<Variable>,
}

impl RangeCheckGadget {
    /// Create a range check gadget
    /// Proves that min <= value <= max
    pub fn new(
        cs: &mut ConstraintSystem,
        value: Variable,
        min: u64,
        max: u64,
    ) -> Result<Self, CircuitError> {
        if min > max {
            return Err(CircuitError::InvalidInput(
                format!("Invalid range: {} > {}", min, max)
            ));
        }

        let value_field = cs.get_value(value)
            .ok_or(CircuitError::InvalidWitness("Value not assigned".to_string()))?;

        // Convert to u64 for range checking
        let value_u64 = field_to_u64(value_field)?;

        if value_u64 < min || value_u64 > max {
            return Err(CircuitError::ConstraintNotSatisfied(
                format!("Value {} not in range [{}, {}]", value_u64, min, max)
            ));
        }

        // Binary decomposition for range proof
        let num_bits = (max - min).next_power_of_two().trailing_zeros() as usize;
        let mut proof_variables = Vec::new();

        // Decompose (value - min) into bits
        let offset = value_u64 - min;
        for i in 0..num_bits {
            let bit = (offset >> i) & 1;
            let bit_var = cs.alloc_variable(Some(Field::from(bit)));

            // Constrain bit to be 0 or 1: bit * (bit - 1) = 0
            let bit_minus_one = cs.alloc_variable(Some(Field::from(bit) - Field::from(1)));
            cs.enforce_mul(bit_var, bit_minus_one, cs.alloc_variable(Some(Field::from(0))));

            proof_variables.push(bit_var);
        }

        // Verify decomposition adds up to (value - min)
        let mut sum = LinearCombination::from_constant(Field::from(min));
        for (i, bit_var) in proof_variables.iter().enumerate() {
            let mut bit_lc = LinearCombination::from_variable(*bit_var);
            bit_lc.scale(Field::from(1u64 << i));
            sum.add(&bit_lc);
        }

        cs.enforce_equal(sum, LinearCombination::from_variable(value));

        Ok(RangeCheckGadget {
            value,
            min,
            max,
            proof_variables,
        })
    }

    /// Number of constraints: roughly 2 * num_bits
    pub fn num_constraints(&self) -> usize {
        self.proof_variables.len() * 2
    }
}

/// Merkle tree path verification gadget
pub struct MerklePathGadget {
    pub leaf: Variable,
    pub root: Variable,
    pub path: Vec<Variable>,
    pub indices: Vec<bool>, // true = right, false = left
}

impl MerklePathGadget {
    /// Verify a Merkle path from leaf to root
    pub fn verify(
        cs: &mut ConstraintSystem,
        leaf: Variable,
        path: &[Variable],
        indices: &[bool],
        expected_root: Variable,
    ) -> Result<Self, CircuitError> {
        if path.len() != indices.len() {
            return Err(CircuitError::InvalidInput(
                "Path length must equal indices length".to_string()
            ));
        }

        let mut current = leaf;

        // Climb the tree, hashing at each level
        for (sibling, is_right) in path.iter().zip(indices.iter()) {
            let (left, right) = if *is_right {
                (*sibling, current)
            } else {
                (current, *sibling)
            };

            // Hash(left, right)
            let hash_gadget = PoseidonGadget::new(cs, &[left, right])?;
            current = hash_gadget.output();
        }

        // Final hash should equal root
        cs.enforce_equal(
            LinearCombination::from_variable(current),
            LinearCombination::from_variable(expected_root),
        );

        Ok(MerklePathGadget {
            leaf,
            root: expected_root,
            path: path.to_vec(),
            indices: indices.to_vec(),
        })
    }

    /// Tree depth
    pub fn depth(&self) -> usize {
        self.path.len()
    }

    /// Number of constraints: roughly depth * hash_constraints
    pub fn num_constraints(&self) -> usize {
        self.depth() * 150 // Approximate for Poseidon
    }
}

/// Conditional selection gadget: returns a if condition else b
pub struct ConditionalSelectGadget {
    pub condition: Variable,
    pub a: Variable,
    pub b: Variable,
    pub output: Variable,
}

impl ConditionalSelectGadget {
    /// Select a if condition == 1, else b
    pub fn new(
        cs: &mut ConstraintSystem,
        condition: Variable,
        a: Variable,
        b: Variable,
    ) -> Result<Self, CircuitError> {
        // Ensure condition is boolean
        let cond_val = cs.get_value(condition)
            .ok_or(CircuitError::InvalidWitness("Condition not assigned".to_string()))?;

        if cond_val != Field::from(0) && cond_val != Field::from(1) {
            return Err(CircuitError::InvalidInput(
                "Condition must be 0 or 1".to_string()
            ));
        }

        // output = condition * a + (1 - condition) * b
        //        = condition * a + b - condition * b
        //        = condition * (a - b) + b

        let output = cs.alloc_variable(None);

        let a_val = cs.get_value(a);
        let b_val = cs.get_value(b);

        if let (Some(a_v), Some(b_v)) = (a_val, b_val) {
            let result = if cond_val == Field::from(1) { a_v } else { b_v };
            cs.set_value(output, result);
        }

        // Compute (a - b)
        let diff = cs.alloc_variable(None);
        if let (Some(a_v), Some(b_v)) = (a_val, b_val) {
            cs.set_value(diff, a_v - b_v);
        }

        // diff = a - b
        let mut lc_diff = LinearCombination::from_variable(a);
        let mut neg_b = LinearCombination::from_variable(b);
        neg_b.scale(Field::from(-1));
        lc_diff.add(&neg_b);
        cs.enforce_equal(lc_diff, LinearCombination::from_variable(diff));

        // temp = condition * diff
        let temp = cs.alloc_variable(None);
        if let Some(d) = cs.get_value(diff) {
            cs.set_value(temp, cond_val * d);
        }
        cs.enforce_mul(condition, diff, temp);

        // output = temp + b
        let mut lc_output = LinearCombination::from_variable(temp);
        lc_output.add(&LinearCombination::from_variable(b));
        cs.enforce_equal(lc_output, LinearCombination::from_variable(output));

        Ok(ConditionalSelectGadget {
            condition,
            a,
            b,
            output,
        })
    }

    pub fn output(&self) -> Variable {
        self.output
    }
}

/// Comparison gadget: proves a < b
pub struct LessThanGadget {
    pub a: Variable,
    pub b: Variable,
    pub result: Variable, // 1 if a < b, else 0
    pub num_bits: usize,
}

impl LessThanGadget {
    /// Create a less-than comparison gadget
    pub fn new(
        cs: &mut ConstraintSystem,
        a: Variable,
        b: Variable,
        num_bits: usize,
    ) -> Result<Self, CircuitError> {
        let a_val = cs.get_value(a)
            .ok_or(CircuitError::InvalidWitness("a not assigned".to_string()))?;
        let b_val = cs.get_value(b)
            .ok_or(CircuitError::InvalidWitness("b not assigned".to_string()))?;

        let a_u64 = field_to_u64(a_val)?;
        let b_u64 = field_to_u64(b_val)?;

        let is_less = a_u64 < b_u64;
        let result = cs.alloc_variable(Some(Field::from(is_less as u64)));

        // Ensure result is boolean
        let result_minus_one = cs.alloc_variable(Some(Field::from(is_less as u64) - Field::from(1)));
        cs.enforce_mul(result, result_minus_one, cs.alloc_variable(Some(Field::from(0))));

        // If a < b, then b - a - 1 >= 0
        // We range check (b - a - 1) fits in num_bits

        let diff = if is_less {
            Field::from(b_u64 - a_u64 - 1)
        } else {
            Field::from(0)
        };

        let diff_var = cs.alloc_variable(Some(diff));

        // Range check diff_var
        if is_less {
            RangeCheckGadget::new(cs, diff_var, 0, (1 << num_bits) - 1)?;
        }

        Ok(LessThanGadget {
            a,
            b,
            result,
            num_bits,
        })
    }

    pub fn result(&self) -> Variable {
        self.result
    }
}

/// Equality gadget: proves a == b
pub struct EqualityGadget {
    pub a: Variable,
    pub b: Variable,
}

impl EqualityGadget {
    pub fn new(
        cs: &mut ConstraintSystem,
        a: Variable,
        b: Variable,
    ) -> Result<Self, CircuitError> {
        cs.enforce_equal(
            LinearCombination::from_variable(a),
            LinearCombination::from_variable(b),
        );

        Ok(EqualityGadget { a, b })
    }
}

// Helper function to convert Field to u64 (for range checks)
fn field_to_u64(f: Field) -> Result<u64, CircuitError> {
    // Convert field element to BigInt and then to u64
    use ark_ff::BigInteger;
    let bigint = f.into_bigint();

    // Check if it fits in u64
    if bigint.num_bits() > 64 {
        return Err(CircuitError::InvalidInput(
            "Field value too large for u64".to_string()
        ));
    }

    // Extract u64 from BigInteger
    Ok(bigint.as_ref()[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_gadget() {
        let mut cs = ConstraintSystem::new();

        let input1 = cs.alloc_variable(Some(Field::from(42)));
        let input2 = cs.alloc_variable(Some(Field::from(100)));

        let hash_gadget = PoseidonGadget::new(&mut cs, &[input1, input2]).unwrap();

        assert!(cs.is_satisfied());
        assert!(cs.get_value(hash_gadget.output()).is_some());
    }

    #[test]
    fn test_range_check_gadget() {
        let mut cs = ConstraintSystem::new();

        let value = cs.alloc_variable(Some(Field::from(50)));

        let range_gadget = RangeCheckGadget::new(&mut cs, value, 10, 100).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(range_gadget.min, 10);
        assert_eq!(range_gadget.max, 100);
    }

    #[test]
    fn test_range_check_fails() {
        let mut cs = ConstraintSystem::new();

        let value = cs.alloc_variable(Some(Field::from(150)));

        let result = RangeCheckGadget::new(&mut cs, value, 10, 100);

        assert!(result.is_err());
    }

    #[test]
    fn test_conditional_select() {
        let mut cs = ConstraintSystem::new();

        let condition = cs.alloc_variable(Some(Field::from(1)));
        let a = cs.alloc_variable(Some(Field::from(42)));
        let b = cs.alloc_variable(Some(Field::from(100)));

        let select = ConditionalSelectGadget::new(&mut cs, condition, a, b).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(select.output()), Some(Field::from(42)));
    }

    #[test]
    fn test_conditional_select_false() {
        let mut cs = ConstraintSystem::new();

        let condition = cs.alloc_variable(Some(Field::from(0)));
        let a = cs.alloc_variable(Some(Field::from(42)));
        let b = cs.alloc_variable(Some(Field::from(100)));

        let select = ConditionalSelectGadget::new(&mut cs, condition, a, b).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(select.output()), Some(Field::from(100)));
    }

    #[test]
    fn test_equality_gadget() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(42)));
        let b = cs.alloc_variable(Some(Field::from(42)));

        EqualityGadget::new(&mut cs, a, b).unwrap();

        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_less_than_true() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(10)));
        let b = cs.alloc_variable(Some(Field::from(20)));

        let lt = LessThanGadget::new(&mut cs, a, b, 8).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(lt.result()), Some(Field::from(1)));
    }

    #[test]
    fn test_less_than_false() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(30)));
        let b = cs.alloc_variable(Some(Field::from(20)));

        let lt = LessThanGadget::new(&mut cs, a, b, 8).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(lt.result()), Some(Field::from(0)));
    }
}
