// Shade Framework - Circuit Primitives
// Copyright (c) 2025 Shadow Protocol Contributors
// Licensed under MIT License

//! Primitive circuit operations and helper functions

use super::*;
use ark_ff::Field as ArkField;

/// Boolean constraint: ensure variable is 0 or 1
pub fn enforce_boolean(
    cs: &mut ConstraintSystem,
    var: Variable,
) -> Result<(), CircuitError> {
    // Boolean constraint: b * (b - 1) = 0
    // This is satisfied when b = 0 or b = 1

    let val = cs.get_value(var)
        .ok_or(CircuitError::InvalidWitness("Variable not assigned".to_string()))?;

    if val != Field::from(0) && val != Field::from(1) {
        return Err(CircuitError::ConstraintNotSatisfied(
            format!("Value must be 0 or 1, got {}", val)
        ));
    }

    let b_minus_1 = cs.alloc_variable(Some(val - Field::from(1)));

    cs.enforce_mul(var, b_minus_1, cs.alloc_variable(Some(Field::from(0))));

    Ok(())
}

/// Addition: c = a + b
pub fn add(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    let a_val = cs.get_value(a);
    let b_val = cs.get_value(b);

    let c = cs.alloc_variable(None);

    if let (Some(a_v), Some(b_v)) = (a_val, b_val) {
        cs.set_value(c, a_v + b_v);
    }

    // Constraint: (a + b) * 1 = c
    let mut sum = LinearCombination::from_variable(a);
    sum.add(&LinearCombination::from_variable(b));

    cs.enforce_constraint(
        sum,
        LinearCombination::from_constant(Field::from(1)),
        LinearCombination::from_variable(c),
    );

    Ok(c)
}

/// Subtraction: c = a - b
pub fn sub(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    let a_val = cs.get_value(a);
    let b_val = cs.get_value(b);

    let c = cs.alloc_variable(None);

    if let (Some(a_v), Some(b_v)) = (a_val, b_val) {
        cs.set_value(c, a_v - b_v);
    }

    // Constraint: (a - b) * 1 = c
    let mut diff = LinearCombination::from_variable(a);
    let mut neg_b = LinearCombination::from_variable(b);
    neg_b.scale(Field::from(-1));
    diff.add(&neg_b);

    cs.enforce_constraint(
        diff,
        LinearCombination::from_constant(Field::from(1)),
        LinearCombination::from_variable(c),
    );

    Ok(c)
}

/// Multiplication: c = a * b
pub fn mul(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    let a_val = cs.get_value(a);
    let b_val = cs.get_value(b);

    let c = cs.alloc_variable(None);

    if let (Some(a_v), Some(b_v)) = (a_val, b_val) {
        cs.set_value(c, a_v * b_v);
    }

    cs.enforce_mul(a, b, c);

    Ok(c)
}

/// Division: c = a / b (requires b != 0)
pub fn div(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    let a_val = cs.get_value(a)
        .ok_or(CircuitError::InvalidWitness("a not assigned".to_string()))?;
    let b_val = cs.get_value(b)
        .ok_or(CircuitError::InvalidWitness("b not assigned".to_string()))?;

    if b_val == Field::from(0) {
        return Err(CircuitError::InvalidInput("Division by zero".to_string()));
    }

    let c_val = a_val / b_val;
    let c = cs.alloc_variable(Some(c_val));

    // Constraint: b * c = a (proves c = a/b)
    cs.enforce_mul(b, c, a);

    Ok(c)
}

/// Inverse: c = 1 / a (requires a != 0)
pub fn inverse(
    cs: &mut ConstraintSystem,
    a: Variable,
) -> Result<Variable, CircuitError> {
    let a_val = cs.get_value(a)
        .ok_or(CircuitError::InvalidWitness("a not assigned".to_string()))?;

    if a_val == Field::from(0) {
        return Err(CircuitError::InvalidInput("Cannot invert zero".to_string()));
    }

    let inv_val = a_val.inverse()
        .ok_or(CircuitError::InvalidInput("Field element not invertible".to_string()))?;

    let inv = cs.alloc_variable(Some(inv_val));

    // Constraint: a * inv = 1
    let one = cs.alloc_variable(Some(Field::from(1)));
    cs.enforce_mul(a, inv, one);

    Ok(inv)
}

/// Square: c = a^2
pub fn square(
    cs: &mut ConstraintSystem,
    a: Variable,
) -> Result<Variable, CircuitError> {
    mul(cs, a, a)
}

/// Constant multiplication: c = a * k
pub fn mul_constant(
    cs: &mut ConstraintSystem,
    a: Variable,
    k: Field,
) -> Result<Variable, CircuitError> {
    let a_val = cs.get_value(a);

    let c = cs.alloc_variable(None);

    if let Some(a_v) = a_val {
        cs.set_value(c, a_v * k);
    }

    // Constraint: a * k = c
    cs.enforce_constraint(
        LinearCombination::from_variable(a),
        LinearCombination::from_constant(k),
        LinearCombination::from_variable(c),
    );

    Ok(c)
}

/// Power: c = a^n (for small n)
pub fn pow(
    cs: &mut ConstraintSystem,
    a: Variable,
    n: u32,
) -> Result<Variable, CircuitError> {
    if n == 0 {
        return Ok(cs.alloc_variable(Some(Field::from(1))));
    }

    if n == 1 {
        return Ok(a);
    }

    // Square-and-multiply algorithm
    let mut result = a;
    let mut base = a;
    let mut exp = n - 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(cs, result, base)?;
        }
        if exp > 1 {
            base = square(cs, base)?;
        }
        exp >>= 1;
    }

    Ok(result)
}

/// Bitwise decomposition: decompose value into bits
pub fn bits_le(
    cs: &mut ConstraintSystem,
    value: Variable,
    num_bits: usize,
) -> Result<Vec<Variable>, CircuitError> {
    let val = cs.get_value(value)
        .ok_or(CircuitError::InvalidWitness("Value not assigned".to_string()))?;

    // Convert to BigInt and extract bits
    use ark_ff::BigInteger;
    let bigint = val.into_bigint();

    let mut bits = Vec::new();

    for i in 0..num_bits {
        let bit = if bigint.get_bit(i) { 1 } else { 0 };
        let bit_var = cs.alloc_variable(Some(Field::from(bit)));

        // Enforce boolean constraint
        enforce_boolean(cs, bit_var)?;

        bits.push(bit_var);
    }

    // Verify bits reconstruct the value
    let mut recomposed = LinearCombination::zero();
    for (i, bit_var) in bits.iter().enumerate() {
        let mut bit_lc = LinearCombination::from_variable(*bit_var);
        bit_lc.scale(Field::from(1u64 << i));
        recomposed.add(&bit_lc);
    }

    cs.enforce_equal(recomposed, LinearCombination::from_variable(value));

    Ok(bits)
}

/// Pack bits into a field element
pub fn pack_bits(
    cs: &mut ConstraintSystem,
    bits: &[Variable],
) -> Result<Variable, CircuitError> {
    let packed = cs.alloc_variable(None);

    // Compute packed value
    let mut packed_val = Field::from(0);
    for (i, bit_var) in bits.iter().enumerate() {
        if let Some(bit) = cs.get_value(*bit_var) {
            packed_val += bit * Field::from(1u64 << i);
        }
    }
    cs.set_value(packed, packed_val);

    // Enforce packing constraint
    let mut lc = LinearCombination::zero();
    for (i, bit_var) in bits.iter().enumerate() {
        let mut bit_lc = LinearCombination::from_variable(*bit_var);
        bit_lc.scale(Field::from(1u64 << i));
        lc.add(&bit_lc);
    }

    cs.enforce_equal(lc, LinearCombination::from_variable(packed));

    Ok(packed)
}

/// XOR of two bits
pub fn xor(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    // XOR = a + b - 2*a*b
    let a_val = cs.get_value(a);
    let b_val = cs.get_value(b);

    // Ensure inputs are bits
    enforce_boolean(cs, a)?;
    enforce_boolean(cs, b)?;

    let result = cs.alloc_variable(None);

    if let (Some(a_v), Some(b_v)) = (a_val, b_val) {
        let xor_val = a_v + b_v - Field::from(2) * a_v * b_v;
        cs.set_value(result, xor_val);
    }

    // Constraint: a + b - 2*a*b = result
    let ab = mul(cs, a, b)?;
    let two_ab = mul_constant(cs, ab, Field::from(2))?;
    let sum = add(cs, a, b)?;
    let xor_result = sub(cs, sum, two_ab)?;

    cs.enforce_equal(
        LinearCombination::from_variable(xor_result),
        LinearCombination::from_variable(result),
    );

    Ok(result)
}

/// AND of two bits
pub fn and(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    // AND = a * b
    enforce_boolean(cs, a)?;
    enforce_boolean(cs, b)?;

    mul(cs, a, b)
}

/// OR of two bits
pub fn or(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    // OR = a + b - a*b
    enforce_boolean(cs, a)?;
    enforce_boolean(cs, b)?;

    let a_val = cs.get_value(a);
    let b_val = cs.get_value(b);

    let result = cs.alloc_variable(None);

    if let (Some(a_v), Some(b_v)) = (a_val, b_val) {
        let or_val = a_v + b_v - a_v * b_v;
        cs.set_value(result, or_val);
    }

    let ab = mul(cs, a, b)?;
    let sum = add(cs, a, b)?;
    let or_result = sub(cs, sum, ab)?;

    cs.enforce_equal(
        LinearCombination::from_variable(or_result),
        LinearCombination::from_variable(result),
    );

    Ok(result)
}

/// NOT of a bit
pub fn not(
    cs: &mut ConstraintSystem,
    a: Variable,
) -> Result<Variable, CircuitError> {
    // NOT = 1 - a
    enforce_boolean(cs, a)?;

    let one = cs.alloc_variable(Some(Field::from(1)));
    sub(cs, one, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(5)));
        let b = cs.alloc_variable(Some(Field::from(7)));

        let c = add(&mut cs, a, b).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(12)));
        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_mul() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(6)));
        let b = cs.alloc_variable(Some(Field::from(7)));

        let c = mul(&mut cs, a, b).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(42)));
        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_div() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(42)));
        let b = cs.alloc_variable(Some(Field::from(7)));

        let c = div(&mut cs, a, b).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(6)));
        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_square() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(9)));

        let c = square(&mut cs, a).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(81)));
        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_pow() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(2)));

        let c = pow(&mut cs, a, 10).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(1024)));
        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_boolean() {
        let mut cs = ConstraintSystem::new();

        let b = cs.alloc_variable(Some(Field::from(1)));
        enforce_boolean(&mut cs, b).unwrap();
        assert!(cs.is_satisfied());

        let mut cs2 = ConstraintSystem::new();
        let b2 = cs2.alloc_variable(Some(Field::from(0)));
        enforce_boolean(&mut cs2, b2).unwrap();
        assert!(cs2.is_satisfied());
    }

    #[test]
    fn test_xor() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(1)));
        let b = cs.alloc_variable(Some(Field::from(0)));

        let c = xor(&mut cs, a, b).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(1)));
        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_and() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(1)));
        let b = cs.alloc_variable(Some(Field::from(1)));

        let c = and(&mut cs, a, b).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(1)));
        assert!(cs.is_satisfied());
    }

    #[test]
    fn test_or() {
        let mut cs = ConstraintSystem::new();

        let a = cs.alloc_variable(Some(Field::from(1)));
        let b = cs.alloc_variable(Some(Field::from(0)));

        let c = or(&mut cs, a, b).unwrap();

        assert_eq!(cs.get_value(c), Some(Field::from(1)));
        assert!(cs.is_satisfied());
    }
}
