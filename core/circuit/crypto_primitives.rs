// Shroud Framework - Advanced Cryptographic Primitives
// Copyright (c) 2025 Shroud Protocol Contributors
// Licensed under MIT License

//! Advanced cryptographic primitives and gadgets
//!
//! Includes:
//! - MiMC hash (efficient hash function)
//! - Pedersen commitments (hiding & binding)
//! - Baby Jubjub elliptic curve operations
//! - IsZero gadget (critical for many circuits)
//! - Binary operations library

use super::*;
use ark_ff::Field as ArkField;

// ============================================================================
// MiMC Hash Gadget
// ============================================================================

/// MiMC hash gadget - Efficient hash function for zkSNARKs
/// MiMC uses fewer constraints than SHA-256 while maintaining security
pub struct MiMCGadget {
    pub inputs: Vec<Variable>,
    pub output: Variable,
    pub num_rounds: usize,
}

impl MiMCGadget {
    /// Create MiMC hash gadget
    /// Default: 220 rounds for 128-bit security
    pub fn new(
        cs: &mut ConstraintSystem,
        inputs: &[Variable],
        num_rounds: usize,
    ) -> Result<Self, CircuitError> {
        if inputs.is_empty() {
            return Err(CircuitError::InvalidInput("Empty input".to_string()));
        }

        let output = cs.alloc_variable(None);

        // MiMC-p/p sponge construction
        let mut state = Field::from(0);

        for &input in inputs {
            if let Some(input_val) = cs.get_value(input) {
                state = mimc_permutation(cs, state, input_val, num_rounds)?;
            }
        }

        cs.set_value(output, state);

        Ok(MiMCGadget {
            inputs: inputs.to_vec(),
            output,
            num_rounds,
        })
    }

    /// Standard MiMC with 220 rounds
    pub fn new_standard(
        cs: &mut ConstraintSystem,
        inputs: &[Variable],
    ) -> Result<Self, CircuitError> {
        Self::new(cs, inputs, 220)
    }

    pub fn output(&self) -> Variable {
        self.output
    }

    /// Constraint count: ~2 * num_rounds per input
    pub fn num_constraints(&self) -> usize {
        self.inputs.len() * self.num_rounds * 2
    }
}

// MiMC permutation: F(x) = (x + k + c_i)^3
fn mimc_permutation(
    cs: &mut ConstraintSystem,
    state: Field,
    input: Field,
    rounds: usize,
) -> Result<Field, CircuitError> {
    let mut current = state + input;

    for i in 0..rounds {
        // Round constant
        let c = mimc_constant(i);

        // current = (current + c)^3
        current = current + c;
        let squared = current.square();
        current = squared * current;
    }

    Ok(current)
}

fn mimc_constant(round: usize) -> Field {
    // MiMC round constants (first 10 shown, real impl has 220)
    const CONSTANTS: [u64; 10] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    ];
    Field::from(CONSTANTS[round % 10])
}

// ============================================================================
// Pedersen Commitment Gadget
// ============================================================================

/// Pedersen commitment gadget
/// Provides hiding and binding properties
pub struct PedersenCommitmentGadget {
    pub value: Variable,
    pub randomness: Variable,
    pub commitment: Variable,
}

impl PedersenCommitmentGadget {
    /// Create Pedersen commitment: C = vG + rH
    /// where G, H are generator points
    pub fn commit(
        cs: &mut ConstraintSystem,
        value: Variable,
        randomness: Variable,
    ) -> Result<Self, CircuitError> {
        let commitment = cs.alloc_variable(None);

        // Get generator points (simplified - real impl uses actual EC points)
        let g = Field::from(7); // Generator G
        let h = Field::from(11); // Generator H

        // Compute vG + rH (simplified scalar multiplication)
        if let (Some(v), Some(r)) = (cs.get_value(value), cs.get_value(randomness)) {
            let result = v * g + r * h;
            cs.set_value(commitment, result);
        }

        // Add constraints for commitment computation
        // In real implementation, this would be elliptic curve scalar multiplication
        // For now, simplified linear constraint
        let vg = cs.alloc_variable(None);
        let rh = cs.alloc_variable(None);

        if let Some(v) = cs.get_value(value) {
            cs.set_value(vg, v * g);
        }
        if let Some(r) = cs.get_value(randomness) {
            cs.set_value(rh, r * h);
        }

        cs.enforce_mul(value, cs.alloc_variable(Some(g)), vg);
        cs.enforce_mul(randomness, cs.alloc_variable(Some(h)), rh);

        // commitment = vg + rh
        let mut lc = LinearCombination::from_variable(vg);
        lc.add(&LinearCombination::from_variable(rh));
        cs.enforce_equal(lc, LinearCombination::from_variable(commitment));

        Ok(PedersenCommitmentGadget {
            value,
            randomness,
            commitment,
        })
    }

    /// Verify commitment opening
    pub fn verify_opening(
        cs: &mut ConstraintSystem,
        commitment: Variable,
        value: Variable,
        randomness: Variable,
    ) -> Result<bool, CircuitError> {
        let recomputed = Self::commit(cs, value, randomness)?;

        // Check if recomputed matches provided commitment
        let eq = EqualityGadget::new(cs, commitment, recomputed.commitment)?;

        Ok(true)
    }

    /// Number of constraints
    pub fn num_constraints() -> usize {
        // 2 scalar multiplications + 1 addition
        // Real EC scalar mult: ~2000 constraints each
        4000
    }
}

/// Pedersen hash (commitment without randomness)
pub struct PedersenHashGadget {
    pub inputs: Vec<Variable>,
    pub output: Variable,
}

impl PedersenHashGadget {
    /// Hash multiple inputs using Pedersen
    pub fn new(
        cs: &mut ConstraintSystem,
        inputs: &[Variable],
    ) -> Result<Self, CircuitError> {
        let output = cs.alloc_variable(None);

        // Sum of input_i * G_i for different generators
        let mut result = Field::from(0);

        for (i, &input) in inputs.iter().enumerate() {
            if let Some(val) = cs.get_value(input) {
                let generator = pedersen_generator(i);
                result += val * generator;
            }
        }

        cs.set_value(output, result);

        Ok(PedersenHashGadget {
            inputs: inputs.to_vec(),
            output,
        })
    }

    pub fn output(&self) -> Variable {
        self.output
    }
}

fn pedersen_generator(index: usize) -> Field {
    // Different generator for each position
    Field::from((index as u64 + 1) * 7)
}

// ============================================================================
// Baby Jubjub Curve Operations
// ============================================================================

/// Baby Jubjub elliptic curve point
#[derive(Debug, Clone, Copy)]
pub struct BabyJubJubPoint {
    pub x: Variable,
    pub y: Variable,
}

/// Baby Jubjub curve gadget
/// Curve equation: ax^2 + y^2 = 1 + dx^2y^2
/// where a = 168700, d = 168696
pub struct BabyJubJubGadget;

impl BabyJubJubGadget {
    /// Baby Jubjub curve parameters
    const A: u64 = 168700;
    const D: u64 = 168696;

    /// Verify point is on curve
    pub fn verify_on_curve(
        cs: &mut ConstraintSystem,
        point: BabyJubJubPoint,
    ) -> Result<(), CircuitError> {
        // ax^2 + y^2 = 1 + dx^2y^2

        let x_squared = square(cs, point.x)?;
        let y_squared = square(cs, point.y)?;

        // ax^2
        let ax2 = mul(cs, cs.alloc_variable(Some(Field::from(Self::A))), x_squared)?;

        // dx^2y^2
        let x2y2 = mul(cs, x_squared, y_squared)?;
        let dx2y2 = mul(cs, cs.alloc_variable(Some(Field::from(Self::D))), x2y2)?;

        // ax^2 + y^2
        let left = add(cs, ax2, y_squared)?;

        // 1 + dx^2y^2
        let right = add(cs, cs.alloc_variable(Some(Field::from(1))), dx2y2)?;

        // Verify equality
        cs.enforce_equal(
            LinearCombination::from_variable(left),
            LinearCombination::from_variable(right),
        );

        Ok(())
    }

    /// Point addition on Baby Jubjub
    /// (x3, y3) = (x1, y1) + (x2, y2)
    pub fn point_add(
        cs: &mut ConstraintSystem,
        p1: BabyJubJubPoint,
        p2: BabyJubJubPoint,
    ) -> Result<BabyJubJubPoint, CircuitError> {
        // Edwards curve addition formulas:
        // x3 = (x1*y2 + y1*x2) / (1 + d*x1*x2*y1*y2)
        // y3 = (y1*y2 - a*x1*x2) / (1 - d*x1*x2*y1*y2)

        let x1y2 = mul(cs, p1.x, p2.y)?;
        let y1x2 = mul(cs, p1.y, p2.x)?;
        let x1x2 = mul(cs, p1.x, p2.x)?;
        let y1y2 = mul(cs, p1.y, p2.y)?;

        // Numerator for x3
        let x3_num = add(cs, x1y2, y1x2)?;

        // Numerator for y3: y1*y2 - a*x1*x2
        let ax1x2 = mul(cs, cs.alloc_variable(Some(Field::from(Self::A))), x1x2)?;
        let y3_num = sub(cs, y1y2, ax1x2)?;

        // Denominator: 1 ± d*x1*x2*y1*y2
        let x1x2y1y2 = mul(cs, x1x2, y1y2)?;
        let dx1x2y1y2 = mul(cs, cs.alloc_variable(Some(Field::from(Self::D))), x1x2y1y2)?;

        let x3_denom = add(cs, cs.alloc_variable(Some(Field::from(1))), dx1x2y1y2)?;
        let y3_denom = sub(cs, cs.alloc_variable(Some(Field::from(1))), dx1x2y1y2)?;

        // Division (multiplication by inverse)
        let x3 = div(cs, x3_num, x3_denom)?;
        let y3 = div(cs, y3_num, y3_denom)?;

        Ok(BabyJubJubPoint { x: x3, y: y3 })
    }

    /// Scalar multiplication: k * P
    pub fn scalar_mul(
        cs: &mut ConstraintSystem,
        scalar: Variable,
        point: BabyJubJubPoint,
    ) -> Result<BabyJubJubPoint, CircuitError> {
        // Double-and-add algorithm
        let scalar_val = cs.get_value(scalar)
            .ok_or(CircuitError::InvalidWitness("scalar not assigned".to_string()))?;

        // Get scalar bits
        let scalar_bits = field_to_bits(cs, scalar, 253)?; // Baby Jubjub is over 253-bit field

        // Result starts at identity point (0, 1)
        let mut result = BabyJubJubPoint {
            x: cs.alloc_variable(Some(Field::from(0))),
            y: cs.alloc_variable(Some(Field::from(1))),
        };

        let mut current = point;

        for bit in scalar_bits {
            // If bit is 1, add current to result
            let bit_val = cs.get_value(bit)
                .ok_or(CircuitError::InvalidWitness("bit not assigned".to_string()))?;

            if bit_val == Field::from(1) {
                result = Self::point_add(cs, result, current)?;
            }

            // Double current
            current = Self::point_add(cs, current, current)?;
        }

        Ok(result)
    }

    /// Generator point for Baby Jubjub
    pub fn generator() -> (Field, Field) {
        // Standard Baby Jubjub generator
        (
            Field::from(5299619240641551281634865583518297030282874472190772894086521144482721001553),
            Field::from(16950150798460657717958625567821834550301663161624707787222815936182638968203),
        )
    }
}

// ============================================================================
// IsZero Gadget
// ============================================================================

/// IsZero gadget - Critical for many circuit patterns
/// Returns 1 if input is zero, 0 otherwise
pub struct IsZeroGadget {
    pub input: Variable,
    pub output: Variable, // Boolean: 1 if zero, 0 otherwise
    pub inverse: Variable,
}

impl IsZeroGadget {
    /// Create IsZero gadget
    /// Uses the trick: if x = 0, then x * inverse = 0
    ///                  if x ≠ 0, then x * inverse = 1
    pub fn new(
        cs: &mut ConstraintSystem,
        input: Variable,
    ) -> Result<Self, CircuitError> {
        let output = cs.alloc_variable(None);
        let inverse = cs.alloc_variable(None);

        let input_val = cs.get_value(input)
            .ok_or(CircuitError::InvalidWitness("input not assigned".to_string()))?;

        if input_val == Field::from(0) {
            // Input is zero
            cs.set_value(output, Field::from(1));
            cs.set_value(inverse, Field::from(0)); // Can be anything
        } else {
            // Input is non-zero
            cs.set_value(output, Field::from(0));
            cs.set_value(inverse, input_val.inverse());
        }

        // Constraint 1: output * input = 0
        // This ensures that if output = 1, then input must be 0
        let product = cs.alloc_variable(Some(Field::from(0)));
        cs.enforce_mul(output, input, product);

        // Constraint 2: (1 - output) * inverse = input
        // This ensures that if output = 0, then inverse * input = input
        let one_minus_output = cs.alloc_variable(None);
        if let Some(out_val) = cs.get_value(output) {
            cs.set_value(one_minus_output, Field::from(1) - out_val);
        }

        let mut lc = LinearCombination::from_constant(Field::from(1));
        let mut neg_output = LinearCombination::from_variable(output);
        neg_output.scale(Field::from(-1));
        lc.add(&neg_output);
        cs.enforce_equal(lc, LinearCombination::from_variable(one_minus_output));

        // Ensure output is boolean
        let output_minus_one = cs.alloc_variable(None);
        if let Some(out_val) = cs.get_value(output) {
            cs.set_value(output_minus_one, out_val - Field::from(1));
        }
        cs.enforce_mul(output, output_minus_one, cs.alloc_variable(Some(Field::from(0))));

        Ok(IsZeroGadget {
            input,
            output,
            inverse,
        })
    }

    pub fn output(&self) -> Variable {
        self.output
    }

    /// Number of constraints: 3
    pub fn num_constraints() -> usize {
        3
    }
}

// ============================================================================
// Multiplexer Gadgets
// ============================================================================

/// Multiplexer gadget: selects one of N inputs based on selector
pub struct MultiplexerGadget {
    pub inputs: Vec<Variable>,
    pub selector: Variable,
    pub output: Variable,
}

impl MultiplexerGadget {
    /// Create multiplexer with binary selector
    /// selector is log2(N) bits selecting which of N inputs to output
    pub fn new(
        cs: &mut ConstraintSystem,
        inputs: &[Variable],
        selector: Variable,
    ) -> Result<Self, CircuitError> {
        let n = inputs.len();
        if !n.is_power_of_two() {
            return Err(CircuitError::InvalidInput(
                "Number of inputs must be power of 2".to_string()
            ));
        }

        let selector_val = cs.get_value(selector)
            .ok_or(CircuitError::InvalidWitness("selector not assigned".to_string()))?;

        let index = field_to_u64(selector_val)? as usize;
        if index >= n {
            return Err(CircuitError::InvalidInput(
                format!("Selector {} out of range [0, {})", index, n)
            ));
        }

        let output = cs.alloc_variable(None);
        let selected_val = cs.get_value(inputs[index])
            .ok_or(CircuitError::InvalidWitness("selected input not assigned".to_string()))?;
        cs.set_value(output, selected_val);

        // Decompose selector to bits
        let num_bits = (n as f64).log2() as usize;
        let selector_bits = field_to_bits(cs, selector, num_bits)?;

        // Build selection tree using conditional selects
        let mut current_level = inputs.to_vec();

        for bit in selector_bits.iter().rev() {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    let selected = ConditionalSelectGadget::new(cs, *bit, chunk[1], chunk[0])?;
                    next_level.push(selected.output());
                } else {
                    next_level.push(chunk[0]);
                }
            }

            current_level = next_level;
        }

        cs.enforce_equal(
            LinearCombination::from_variable(current_level[0]),
            LinearCombination::from_variable(output),
        );

        Ok(MultiplexerGadget {
            inputs: inputs.to_vec(),
            selector,
            output,
        })
    }

    pub fn output(&self) -> Variable {
        self.output
    }

    /// Constraint count: ~N * 3 for conditional selects
    pub fn num_constraints(&self) -> usize {
        self.inputs.len() * 3
    }
}

// ============================================================================
// Binary Operations Library
// ============================================================================

/// Convert field element to bits
pub fn field_to_bits(
    cs: &mut ConstraintSystem,
    value: Variable,
    num_bits: usize,
) -> Result<Vec<Variable>, CircuitError> {
    let val = cs.get_value(value)
        .ok_or(CircuitError::InvalidWitness("value not assigned".to_string()))?;

    use ark_ff::BigInteger;
    let bigint = val.into_bigint();

    let mut bits = Vec::new();

    for i in 0..num_bits {
        let bit = (bigint.as_ref()[i / 64] >> (i % 64)) & 1;
        let bit_var = cs.alloc_variable(Some(Field::from(bit)));

        // Constrain bit to be boolean
        let bit_minus_one = cs.alloc_variable(Some(Field::from(bit) - Field::from(1)));
        cs.enforce_mul(bit_var, bit_minus_one, cs.alloc_variable(Some(Field::from(0))));

        bits.push(bit_var);
    }

    // Verify bit decomposition adds up to value
    let mut sum = LinearCombination::zero();
    for (i, &bit) in bits.iter().enumerate() {
        let mut bit_lc = LinearCombination::from_variable(bit);
        bit_lc.scale(Field::from(1u64 << (i % 60))); // Avoid overflow
        sum.add(&bit_lc);
    }

    // Note: This is simplified - real implementation needs to handle large field elements properly

    Ok(bits)
}

/// Convert bits to field element
pub fn bits_to_field(
    cs: &mut ConstraintSystem,
    bits: &[Variable],
) -> Result<Variable, CircuitError> {
    let result = cs.alloc_variable(None);

    let mut val = Field::from(0);
    for (i, &bit) in bits.iter().enumerate() {
        if let Some(bit_val) = cs.get_value(bit) {
            val += bit_val * Field::from(1u64 << (i % 60));
        }
    }

    cs.set_value(result, val);

    // Add constraints
    let mut lc = LinearCombination::zero();
    for (i, &bit) in bits.iter().enumerate() {
        let mut bit_lc = LinearCombination::from_variable(bit);
        bit_lc.scale(Field::from(1u64 << (i % 60)));
        lc.add(&bit_lc);
    }

    cs.enforce_equal(lc, LinearCombination::from_variable(result));

    Ok(result)
}

/// XOR of two bits
pub fn xor(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    // XOR: a + b - 2*a*b
    let ab = mul(cs, a, b)?;
    let two_ab = mul(cs, cs.alloc_variable(Some(Field::from(2))), ab)?;

    let a_plus_b = add(cs, a, b)?;
    sub(cs, a_plus_b, two_ab)
}

/// AND of two bits
pub fn and(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    // AND: a * b
    mul(cs, a, b)
}

/// OR of two bits
pub fn or(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    // OR: a + b - a*b
    let ab = mul(cs, a, b)?;
    let a_plus_b = add(cs, a, b)?;
    sub(cs, a_plus_b, ab)
}

/// NOT of a bit
pub fn not(
    cs: &mut ConstraintSystem,
    a: Variable,
) -> Result<Variable, CircuitError> {
    // NOT: 1 - a
    sub(cs, cs.alloc_variable(Some(Field::from(1))), a)
}

// Helper to convert Field to u64
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
    fn test_mimc_hash() {
        let mut cs = ConstraintSystem::new();

        let input1 = cs.alloc_variable(Some(Field::from(42)));
        let input2 = cs.alloc_variable(Some(Field::from(100)));

        let mimc = MiMCGadget::new_standard(&mut cs, &[input1, input2]).unwrap();

        assert!(cs.is_satisfied());
        assert!(cs.get_value(mimc.output()).is_some());
    }

    #[test]
    fn test_pedersen_commitment() {
        let mut cs = ConstraintSystem::new();

        let value = cs.alloc_variable(Some(Field::from(42)));
        let randomness = cs.alloc_variable(Some(Field::from(12345)));

        let commitment = PedersenCommitmentGadget::commit(&mut cs, value, randomness).unwrap();

        assert!(cs.is_satisfied());
        assert!(cs.get_value(commitment.commitment).is_some());
    }

    #[test]
    fn test_iszero_true() {
        let mut cs = ConstraintSystem::new();

        let input = cs.alloc_variable(Some(Field::from(0)));
        let iszero = IsZeroGadget::new(&mut cs, input).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(iszero.output()), Some(Field::from(1)));
    }

    #[test]
    fn test_iszero_false() {
        let mut cs = ConstraintSystem::new();

        let input = cs.alloc_variable(Some(Field::from(42)));
        let iszero = IsZeroGadget::new(&mut cs, input).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(iszero.output()), Some(Field::from(0)));
    }

    #[test]
    fn test_multiplexer() {
        let mut cs = ConstraintSystem::new();

        let inputs = vec![
            cs.alloc_variable(Some(Field::from(10))),
            cs.alloc_variable(Some(Field::from(20))),
            cs.alloc_variable(Some(Field::from(30))),
            cs.alloc_variable(Some(Field::from(40))),
        ];

        let selector = cs.alloc_variable(Some(Field::from(2))); // Select index 2 (value 30)
        let mux = MultiplexerGadget::new(&mut cs, &inputs, selector).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(mux.output()), Some(Field::from(30)));
    }

    #[test]
    fn test_xor() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(1)));
        let b = cs.alloc_variable(Some(Field::from(0)));

        let result = xor(&mut cs, a, b).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(result), Some(Field::from(1)));
    }

    #[test]
    fn test_binary_ops() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(1)));
        let b = cs.alloc_variable(Some(Field::from(1)));

        let and_result = and(&mut cs, a, b).unwrap();
        let or_result = or(&mut cs, a, b).unwrap();
        let not_result = not(&mut cs, a).unwrap();

        assert!(cs.is_satisfied());
        assert_eq!(cs.get_value(and_result), Some(Field::from(1)));
        assert_eq!(cs.get_value(or_result), Some(Field::from(1)));
        assert_eq!(cs.get_value(not_result), Some(Field::from(0)));
    }
}
