// Shroud Framework - Procedural Macro Example
// Copyright (c) 2025 Shroud Protocol Contributors
// Licensed under MIT License

//! Example demonstrating the use of Shroud procedural macros
//! for ergonomic circuit development

use shroud::prelude::*;
use shade_macros::*;

/// Simple square circuit using derive macro
#[derive(Circuit)]
pub struct SquareCircuit {
    #[private]
    pub input: Field,

    #[public]
    pub output: Field,
}

impl SquareCircuit {
    /// Constraint implementation
    pub fn constraints(&self, cs: &mut ConstraintSystem) -> Result<(), CircuitError> {
        // Allocate variables
        let input = cs.alloc_variable(Some(self.input));
        let output = cs.alloc_variable(Some(self.output));

        // Constraint: output = input * input
        let squared = mul(cs, input, input)?;
        cs.enforce_equal(
            LinearCombination::from_variable(squared),
            LinearCombination::from_variable(output),
        );

        Ok(())
    }
}

/// Hash preimage circuit with macros
#[derive(Circuit)]
pub struct HashPreimageCircuit {
    #[private]
    pub preimage: Field,

    #[public]
    pub hash: Field,

    #[constant]
    pub salt: Field,
}

impl HashPreimageCircuit {
    pub fn constraints(&self, cs: &mut ConstraintSystem) -> Result<(), CircuitError> {
        let preimage = cs.alloc_variable(Some(self.preimage));
        let salt = cs.alloc_variable(Some(self.salt));
        let hash = cs.alloc_variable(Some(self.hash));

        // Compute salted hash
        let salted = add(cs, preimage, salt)?;
        let computed_hash = poseidon_hash(cs, &[salted])?;

        // Verify hash matches
        cs.enforce_equal(
            LinearCombination::from_variable(computed_hash),
            LinearCombination::from_variable(hash),
        );

        Ok(())
    }
}

/// Merkle proof circuit
#[derive(Circuit)]
pub struct MerkleProofCircuit {
    #[private]
    pub leaf: Field,

    #[private]
    pub path: Vec<Field>,

    #[private]
    pub indices: Vec<bool>,

    #[public]
    pub root: Field,
}

impl MerkleProofCircuit {
    pub fn constraints(&self, cs: &mut ConstraintSystem) -> Result<(), CircuitError> {
        let leaf = cs.alloc_variable(Some(self.leaf));
        let root = cs.alloc_variable(Some(self.root));

        // Allocate path variables
        let path_vars: Vec<Variable> = self.path
            .iter()
            .map(|&val| cs.alloc_variable(Some(val)))
            .collect();

        // Verify Merkle path
        let merkle_gadget = MerklePathGadget::verify(
            cs,
            leaf,
            &path_vars,
            &self.indices,
            root,
        )?;

        Ok(())
    }
}

/// Range proof circuit
#[derive(Circuit)]
pub struct RangeProofCircuit {
    #[private]
    pub value: Field,

    #[public]
    pub min: Field,

    #[public]
    pub max: Field,

    #[constant]
    pub num_bits: usize,
}

impl RangeProofCircuit {
    pub fn constraints(&self, cs: &mut ConstraintSystem) -> Result<(), CircuitError> {
        let value = cs.alloc_variable(Some(self.value));

        // Range check: min <= value <= max
        let min_u64 = field_to_u64(self.min)?;
        let max_u64 = field_to_u64(self.max)?;

        RangeCheckGadget::new(cs, value, min_u64, max_u64)?;

        Ok(())
    }
}

// Helper functions with gadget macro

/// Poseidon hash gadget (example)
pub fn poseidon_hash(cs: &mut ConstraintSystem, inputs: &[Variable]) -> Result<Variable, CircuitError> {
    let gadget = PoseidonGadget::new(cs, inputs)?;
    Ok(gadget.output())
}

// Example usage

fn main() {
    println!("===========================================");
    println!("Shroud Framework - Procedural Macro Example");
    println!("===========================================\n");

    // Example 1: Square circuit
    {
        println!("1. Square Circuit");
        println!("   Proving: output = input²\n");

        let circuit = SquareCircuit {
            input: Field::from(5),
            output: Field::from(25),
        };

        println!("   Private fields: {:?}", SquareCircuit::private_fields());
        println!("   Public fields: {:?}", SquareCircuit::public_fields());

        let mut cs = ConstraintSystem::new();
        circuit.build_constraints(&mut cs).unwrap();

        println!("   ✓ Constraints: {}", cs.num_constraints());
        println!("   ✓ Satisfied: {}\n", cs.is_satisfied());
    }

    // Example 2: Hash preimage
    {
        println!("2. Hash Preimage Circuit");
        println!("   Proving: hash = H(preimage + salt)\n");

        let preimage = Field::from(12345);
        let salt = Field::from(99999);
        let hash = compute_hash(preimage, salt);

        let circuit = HashPreimageCircuit {
            preimage,
            hash,
            salt,
        };

        let mut cs = ConstraintSystem::new();
        circuit.build_constraints(&mut cs).unwrap();

        println!("   Private: preimage");
        println!("   Public: hash");
        println!("   Constant: salt");
        println!("   ✓ Constraints: {}", cs.num_constraints());
        println!("   ✓ Satisfied: {}\n", cs.is_satisfied());
    }

    // Example 3: Merkle proof
    {
        println!("3. Merkle Proof Circuit");
        println!("   Proving: leaf is in Merkle tree\n");

        let leaf = Field::from(42);
        let path = vec![
            Field::from(100),
            Field::from(200),
            Field::from(300),
        ];
        let indices = vec![false, true, false];
        let root = compute_merkle_root(leaf, &path, &indices);

        let circuit = MerkleProofCircuit {
            leaf,
            path,
            indices,
            root,
        };

        let mut cs = ConstraintSystem::new();
        circuit.build_constraints(&mut cs).unwrap();

        println!("   Tree depth: 3");
        println!("   ✓ Constraints: {}", cs.num_constraints());
        println!("   ✓ Satisfied: {}\n", cs.is_satisfied());
    }

    // Example 4: Range proof
    {
        println!("4. Range Proof Circuit");
        println!("   Proving: 18 <= value <= 65\n");

        let circuit = RangeProofCircuit {
            value: Field::from(25),
            min: Field::from(18),
            max: Field::from(65),
            num_bits: 8,
        };

        let mut cs = ConstraintSystem::new();
        circuit.build_constraints(&mut cs).unwrap();

        println!("   Value: 25");
        println!("   Range: [18, 65]");
        println!("   ✓ Constraints: {}", cs.num_constraints());
        println!("   ✓ Satisfied: {}\n", cs.is_satisfied());
    }

    println!("===========================================");
    println!("All macro examples completed successfully!");
    println!("===========================================");
}

// Helper functions

fn compute_hash(preimage: Field, salt: Field) -> Field {
    // Simplified hash computation
    (preimage + salt).square()
}

fn compute_merkle_root(leaf: Field, path: &[Field], indices: &[bool]) -> Field {
    let mut current = leaf;

    for (sibling, is_right) in path.iter().zip(indices.iter()) {
        let (left, right) = if *is_right {
            (*sibling, current)
        } else {
            (current, *sibling)
        };
        // Hash pair
        current = (left + right).square();
    }

    current
}

fn field_to_u64(f: Field) -> Result<u64, CircuitError> {
    use ark_ff::BigInteger;
    let bigint = f.into_bigint();

    if bigint.num_bits() > 64 {
        return Err(CircuitError::InvalidInput(
            "Field value too large for u64".to_string()
        ));
    }

    Ok(bigint.as_ref()[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_circuit_macro() {
        let circuit = SquareCircuit {
            input: Field::from(7),
            output: Field::from(49),
        };

        let mut cs = ConstraintSystem::new();
        circuit.build_constraints(&mut cs).unwrap();

        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_hash_preimage_macro() {
        let preimage = Field::from(100);
        let salt = Field::from(200);
        let hash = compute_hash(preimage, salt);

        let circuit = HashPreimageCircuit {
            preimage,
            hash,
            salt,
        };

        let mut cs = ConstraintSystem::new();
        circuit.build_constraints(&mut cs).unwrap();

        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_field_reflection() {
        // Test that derive macro generates correct metadata
        assert_eq!(SquareCircuit::private_fields(), &["input"]);
        assert_eq!(SquareCircuit::public_fields(), &["output"]);

        assert_eq!(HashPreimageCircuit::private_fields(), &["preimage"]);
        assert_eq!(HashPreimageCircuit::public_fields(), &["hash"]);
        assert_eq!(HashPreimageCircuit::constant_fields(), &["salt"]);
    }
}
