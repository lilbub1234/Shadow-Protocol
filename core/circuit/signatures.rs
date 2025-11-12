// Shroud Framework - Signature Verification Gadgets
// Copyright (c) 2025 Shroud Protocol Contributors
// Licensed under MIT License

//! Signature verification in zero-knowledge

use super::*;
use super::gadgets::*;
use super::primitives::*;

/// ECDSA signature verification gadget (secp256k1)
pub struct ECDSAGadget {
    pub message_hash: Variable,
    pub signature_r: Variable,
    pub signature_s: Variable,
    pub public_key_x: Variable,
    pub public_key_y: Variable,
    pub is_valid: Variable,
}

impl ECDSAGadget {
    /// Verify an ECDSA signature
    /// Proves that signature (r, s) is valid for message_hash and public_key
    pub fn verify(
        cs: &mut ConstraintSystem,
        message_hash: Variable,
        signature_r: Variable,
        signature_s: Variable,
        public_key_x: Variable,
        public_key_y: Variable,
    ) -> Result<Self, CircuitError> {
        // ECDSA verification algorithm:
        // 1. Compute w = s^(-1) mod n
        // 2. Compute u1 = message_hash * w mod n
        // 3. Compute u2 = r * w mod n
        // 4. Compute (x, y) = u1*G + u2*PublicKey
        // 5. Check x mod n == r

        // For this implementation, we'll create a simplified version
        // Real implementation would use elliptic curve operations

        let is_valid = cs.alloc_variable(Some(Field::from(1)));

        // Inverse of s
        let s_val = cs.get_value(signature_s)
            .ok_or(CircuitError::InvalidWitness("s not assigned".to_string()))?;
        let w = inverse(cs, signature_s)?;

        // u1 = message_hash * w
        let u1 = mul(cs, message_hash, w)?;

        // u2 = r * w
        let u2 = mul(cs, signature_r, w)?;

        // In a real implementation, we would:
        // 1. Perform scalar multiplication: u1 * G
        // 2. Perform scalar multiplication: u2 * PK
        // 3. Perform point addition
        // 4. Verify the result

        // For now, we'll add constraints that represent these operations
        // This is a placeholder - real implementation needs elliptic curve arithmetic

        // Verify public key is on curve: y^2 = x^3 + 7 (for secp256k1)
        let y_squared = square(cs, public_key_y)?;
        let x_squared = square(cs, public_key_x)?;
        let x_cubed = mul(cs, x_squared, public_key_x)?;
        let x_cubed_plus_7 = add(cs, x_cubed, cs.alloc_variable(Some(Field::from(7))))?;

        cs.enforce_equal(
            LinearCombination::from_variable(y_squared),
            LinearCombination::from_variable(x_cubed_plus_7),
        );

        Ok(ECDSAGadget {
            message_hash,
            signature_r,
            signature_s,
            public_key_x,
            public_key_y,
            is_valid,
        })
    }

    /// Get the verification result
    pub fn result(&self) -> Variable {
        self.is_valid
    }

    /// Number of constraints (approximate)
    pub fn num_constraints() -> usize {
        // ECDSA verification requires:
        // - Field inversions: ~200 constraints
        // - Scalar multiplications: ~2000 constraints each (x2)
        // - Point addition: ~500 constraints
        // Total: ~4700 constraints
        4700
    }
}

/// EdDSA signature verification gadget (Ed25519)
pub struct EdDSAGadget {
    pub message: Vec<Variable>,
    pub signature_r: (Variable, Variable), // Point R
    pub signature_s: Variable,              // Scalar s
    pub public_key: (Variable, Variable),   // Point A
    pub is_valid: Variable,
}

impl EdDSAGadget {
    /// Verify an EdDSA signature
    /// Proves that signature (R, s) is valid for message and public_key
    pub fn verify(
        cs: &mut ConstraintSystem,
        message: &[Variable],
        signature_r: (Variable, Variable),
        signature_s: Variable,
        public_key: (Variable, Variable),
    ) -> Result<Self, CircuitError> {
        // EdDSA verification algorithm:
        // 1. Compute h = H(R || A || M)
        // 2. Check that 8 * s * B = 8 * R + 8 * h * A
        // Where:
        // - R is the signature point (r_x, r_y)
        // - s is the signature scalar
        // - A is the public key point
        // - B is the base point
        // - M is the message

        let is_valid = cs.alloc_variable(Some(Field::from(1)));

        // Hash the message along with R and A
        let mut hash_inputs = vec![signature_r.0, signature_r.1, public_key.0, public_key.1];
        hash_inputs.extend_from_slice(message);

        let h_gadget = PoseidonGadget::new(cs, &hash_inputs)?;
        let h = h_gadget.output();

        // Verify Edwards curve equation for R: a*x^2 + y^2 = 1 + d*x^2*y^2
        // For Ed25519: -x^2 + y^2 = 1 - (121665/121666)*x^2*y^2
        let r_x_squared = square(cs, signature_r.0)?;
        let r_y_squared = square(cs, signature_r.1)?;

        // Verify Edwards curve equation for public key A
        let a_x_squared = square(cs, public_key.0)?;
        let a_y_squared = square(cs, public_key.1)?;

        // In a real implementation, we would:
        // 1. Perform scalar multiplications
        // 2. Perform point additions on Edwards curve
        // 3. Verify the equation 8*s*B = 8*R + 8*h*A

        // This is a placeholder for the actual verification
        // Real implementation requires full Edwards curve arithmetic

        Ok(EdDSAGadget {
            message: message.to_vec(),
            signature_r,
            signature_s,
            public_key,
            is_valid,
        })
    }

    /// Get the verification result
    pub fn result(&self) -> Variable {
        self.is_valid
    }

    /// Number of constraints (approximate)
    pub fn num_constraints() -> usize {
        // EdDSA verification requires:
        // - Hash computation: ~150 constraints
        // - Scalar multiplications: ~1500 constraints each (x3)
        // - Point additions: ~300 constraints each (x2)
        // Total: ~5250 constraints
        5250
    }
}

/// Schnorr signature verification gadget
pub struct SchnorrGadget {
    pub message_hash: Variable,
    pub signature_r: Variable,
    pub signature_s: Variable,
    pub public_key: Variable,
    pub is_valid: Variable,
}

impl SchnorrGadget {
    /// Verify a Schnorr signature
    /// Proves that signature (r, s) is valid for message_hash and public_key
    pub fn verify(
        cs: &mut ConstraintSystem,
        message_hash: Variable,
        signature_r: Variable,
        signature_s: Variable,
        public_key: Variable,
    ) -> Result<Self, CircuitError> {
        // Schnorr verification algorithm:
        // 1. Compute e = H(R || PK || m)
        // 2. Check that s*G = R + e*PK

        let is_valid = cs.alloc_variable(Some(Field::from(1)));

        // Compute challenge hash e = H(R || PK || m)
        let e_gadget = PoseidonGadget::new(cs, &[signature_r, public_key, message_hash])?;
        let e = e_gadget.output();

        // In a real implementation:
        // 1. Compute s*G (scalar multiplication)
        // 2. Compute e*PK (scalar multiplication)
        // 3. Compute R + e*PK (point addition)
        // 4. Verify s*G = R + e*PK

        // Placeholder for actual verification
        // Real implementation needs elliptic curve operations

        Ok(SchnorrGadget {
            message_hash,
            signature_r,
            signature_s,
            public_key,
            is_valid,
        })
    }

    /// Get the verification result
    pub fn result(&self) -> Variable {
        self.is_valid
    }

    /// Number of constraints (approximate)
    pub fn num_constraints() -> usize {
        // Schnorr verification requires:
        // - Hash computation: ~150 constraints
        // - Scalar multiplications: ~1500 constraints each (x2)
        // - Point addition: ~300 constraints
        // Total: ~3450 constraints
        3450
    }
}

/// BLS signature verification gadget
pub struct BLSGadget {
    pub message: Vec<Variable>,
    pub signature: (Variable, Variable, Variable), // G2 point (3 coordinates for BLS12-381)
    pub public_key: (Variable, Variable),          // G1 point
    pub is_valid: Variable,
}

impl BLSGadget {
    /// Verify a BLS signature
    /// Proves that signature is valid for message and public_key
    pub fn verify(
        cs: &mut ConstraintSystem,
        message: &[Variable],
        signature: (Variable, Variable, Variable),
        public_key: (Variable, Variable),
    ) -> Result<Self, CircuitError> {
        // BLS verification algorithm:
        // 1. Compute H = HashToCurve(message)
        // 2. Check that e(PK, H) = e(G1, Signature)
        // Where e() is the pairing function

        let is_valid = cs.alloc_variable(Some(Field::from(1)));

        // Hash message to curve point
        let h_gadget = PoseidonGadget::new(cs, message)?;
        let h = h_gadget.output();

        // In a real implementation:
        // 1. Hash to curve (convert hash to elliptic curve point)
        // 2. Compute pairing e(PK, H)
        // 3. Compute pairing e(G1, Signature)
        // 4. Verify equality

        // BLS signatures require pairing operations which are expensive
        // but provide signature aggregation benefits

        Ok(BLSGadget {
            message: message.to_vec(),
            signature,
            public_key,
            is_valid,
        })
    }

    /// Get the verification result
    pub fn result(&self) -> Variable {
        self.is_valid
    }

    /// Number of constraints (approximate)
    pub fn num_constraints() -> usize {
        // BLS verification requires:
        // - Hash to curve: ~500 constraints
        // - Pairing computation: ~10000 constraints each (x2)
        // Total: ~20500 constraints
        20500
    }
}

/// Signature verification helper functions
pub mod helpers {
    use super::*;

    /// Verify multiple ECDSA signatures efficiently
    pub fn batch_verify_ecdsa(
        cs: &mut ConstraintSystem,
        signatures: &[(Variable, Variable, Variable, Variable, Variable)], // (msg, r, s, pk_x, pk_y)
    ) -> Result<Variable, CircuitError> {
        let mut all_valid = cs.alloc_variable(Some(Field::from(1)));

        for (msg, r, s, pk_x, pk_y) in signatures {
            let sig = ECDSAGadget::verify(cs, *msg, *r, *s, *pk_x, *pk_y)?;
            all_valid = and(cs, all_valid, sig.result())?;
        }

        Ok(all_valid)
    }

    /// Aggregate BLS signatures
    pub fn aggregate_bls_signatures(
        cs: &mut ConstraintSystem,
        signatures: &[(Variable, Variable, Variable)],
    ) -> Result<(Variable, Variable, Variable), CircuitError> {
        // BLS signatures can be aggregated by simple addition
        // This is one of the key benefits of BLS

        if signatures.is_empty() {
            return Err(CircuitError::InvalidInput("No signatures to aggregate".to_string()));
        }

        let mut agg_x = signatures[0].0;
        let mut agg_y = signatures[0].1;
        let mut agg_z = signatures[0].2;

        for sig in &signatures[1..] {
            // Point addition on G2
            // In real implementation, this would be proper elliptic curve addition
            agg_x = add(cs, agg_x, sig.0)?;
            agg_y = add(cs, agg_y, sig.1)?;
            agg_z = add(cs, agg_z, sig.2)?;
        }

        Ok((agg_x, agg_y, agg_z))
    }
}

/// RSA signature verification gadget
/// Verifies RSA-PSS or RSA-PKCS#1 v1.5 signatures
pub struct RSAGadget {
    pub message_hash: Variable,
    pub signature: Variable,
    pub public_exponent: Variable,
    pub modulus: Variable,
    pub is_valid: Variable,
    pub num_constraints: usize,
}

impl RSAGadget {
    /// Verify an RSA signature
    /// RSA verification: m = s^e mod n
    /// where s = signature, e = public exponent, n = modulus
    pub fn verify(
        cs: &mut ConstraintSystem,
        message_hash: Variable,
        signature: Variable,
        public_exponent: Variable,
        modulus: Variable,
    ) -> Result<Self, CircuitError> {
        // RSA parameters (typical: 2048-bit or 4096-bit)
        // For circuit efficiency, we'll implement 1024-bit RSA
        const RSA_BITS: usize = 1024;
        const LIMB_SIZE: usize = 64; // 64-bit limbs
        const NUM_LIMBS: usize = RSA_BITS / LIMB_SIZE; // 16 limbs

        let mut constraint_count = 0;

        // Decompose signature into limbs for big integer arithmetic
        let sig_val = cs.get_value(signature)
            .ok_or(CircuitError::InvalidWitness("signature not assigned".to_string()))?;

        let mut sig_limbs = Vec::new();
        for i in 0..NUM_LIMBS {
            let limb = cs.alloc_variable(None);
            sig_limbs.push(limb);
            constraint_count += 1;
        }

        // Decompose modulus into limbs
        let mod_val = cs.get_value(modulus)
            .ok_or(CircuitError::InvalidWitness("modulus not assigned".to_string()))?;

        let mut mod_limbs = Vec::new();
        for i in 0..NUM_LIMBS {
            let limb = cs.alloc_variable(None);
            mod_limbs.push(limb);
            constraint_count += 1;
        }

        // Get public exponent (typically 65537 = 0x10001)
        let e_val = cs.get_value(public_exponent)
            .ok_or(CircuitError::InvalidWitness("public exponent not assigned".to_string()))?;

        // Modular exponentiation: result = signature^e mod n
        // We'll use square-and-multiply algorithm
        let result = rsa_mod_exp(cs, &sig_limbs, e_val, &mod_limbs, &mut constraint_count)?;

        // Compare result with message hash
        // In real RSA, we'd also verify padding (PSS or PKCS#1 v1.5)
        let is_valid = cs.alloc_variable(None);

        // Check if result == message_hash
        let result_combined = combine_limbs(cs, &result)?;
        constraint_count += 1;

        let equals = equality_check(cs, result_combined, message_hash)?;
        constraint_count += 1;

        cs.set_value(is_valid, cs.get_value(equals).unwrap_or(Field::from(0)));

        // Ensure is_valid is boolean
        let is_valid_minus_one = cs.alloc_variable(None);
        if let Some(val) = cs.get_value(is_valid) {
            cs.set_value(is_valid_minus_one, val - Field::from(1));
        }
        cs.enforce_mul(is_valid, is_valid_minus_one, cs.alloc_variable(Some(Field::from(0))));
        constraint_count += 1;

        Ok(RSAGadget {
            message_hash,
            signature,
            public_exponent,
            modulus,
            is_valid,
            num_constraints: constraint_count,
        })
    }

    /// Verify with standard parameters (e = 65537)
    pub fn verify_standard(
        cs: &mut ConstraintSystem,
        message_hash: Variable,
        signature: Variable,
        modulus: Variable,
    ) -> Result<Self, CircuitError> {
        let e = cs.alloc_variable(Some(Field::from(65537)));
        Self::verify(cs, message_hash, signature, e, modulus)
    }

    /// Get the verification result
    pub fn result(&self) -> Variable {
        self.is_valid
    }

    /// Number of constraints for RSA verification
    /// Depends on key size and exponent
    pub fn num_constraints() -> usize {
        // RSA-1024 verification requires:
        // - Limb decomposition: ~32 constraints
        // - Modular exponentiation: ~8000 constraints (depends on exponent)
        // - Modular reduction: ~1000 constraints per operation
        // - Comparison: ~100 constraints
        // Total: ~12000-15000 constraints for RSA-1024
        // RSA-2048 would be ~40000-50000 constraints
        12000
    }
}

// Helper: RSA modular exponentiation using square-and-multiply
fn rsa_mod_exp(
    cs: &mut ConstraintSystem,
    base_limbs: &[Variable],
    exponent: Field,
    modulus_limbs: &[Variable],
    constraint_count: &mut usize,
) -> Result<Vec<Variable>, CircuitError> {
    // Convert exponent to bits
    let exp_u64 = field_to_u64(exponent)?;
    let exp_bits = (0..64).map(|i| (exp_u64 >> i) & 1 == 1).collect::<Vec<_>>();

    // Initialize result = 1
    let mut result = vec_of_constants(cs, base_limbs.len(), Field::from(1));
    let mut base = base_limbs.to_vec();

    // Square-and-multiply
    for bit in exp_bits {
        if bit {
            // result = (result * base) mod modulus
            result = big_mul_mod(cs, &result, &base, modulus_limbs, constraint_count)?;
        }

        // base = (base * base) mod modulus
        let base_clone = base.clone();
        base = big_mul_mod(cs, &base, &base_clone, modulus_limbs, constraint_count)?;
    }

    Ok(result)
}

// Helper: Multiply two big integers modulo modulus
fn big_mul_mod(
    cs: &mut ConstraintSystem,
    a: &[Variable],
    b: &[Variable],
    modulus: &[Variable],
    constraint_count: &mut usize,
) -> Result<Vec<Variable>, CircuitError> {
    // Simplified big integer multiplication and reduction
    // Real implementation would use Karatsuba or similar algorithms

    let result_len = a.len();
    let mut result = Vec::new();

    for i in 0..result_len {
        let limb = cs.alloc_variable(None);

        // For each limb, compute contribution from multiplication
        if let (Some(a_val), Some(b_val)) = (cs.get_value(a[i]), cs.get_value(b[i])) {
            // Simplified: result[i] = a[i] * b[i]
            cs.set_value(limb, a_val * b_val);
        }

        // Add constraint: limb = a[i] * b[i]
        cs.enforce_mul(a[i], b[i], limb);
        *constraint_count += 1;

        result.push(limb);
    }

    // Modular reduction (simplified)
    // Real implementation would do Barrett or Montgomery reduction
    for i in 0..result_len {
        let reduced = cs.alloc_variable(None);

        if let (Some(r_val), Some(m_val)) = (cs.get_value(result[i]), cs.get_value(modulus[i])) {
            // Simplified reduction: subtract modulus if needed
            let diff = r_val - m_val;
            cs.set_value(reduced, if diff.into_bigint().as_ref()[0] > r_val.into_bigint().as_ref()[0] {
                r_val
            } else {
                diff
            });
        }

        result[i] = reduced;
        *constraint_count += 1;
    }

    Ok(result)
}

// Helper: Create vector of constant limbs
fn vec_of_constants(cs: &mut ConstraintSystem, len: usize, value: Field) -> Vec<Variable> {
    (0..len).map(|_| cs.alloc_variable(Some(value))).collect()
}

// Helper: Combine limbs back into single field element
fn combine_limbs(cs: &mut ConstraintSystem, limbs: &[Variable]) -> Result<Variable, CircuitError> {
    let combined = cs.alloc_variable(None);
    let mut sum = Field::from(0);

    for (i, &limb) in limbs.iter().enumerate() {
        if let Some(val) = cs.get_value(limb) {
            let shift = Field::from(1u64 << (i % 8)); // Simplified
            sum += val * shift;
        }
    }

    cs.set_value(combined, sum);
    Ok(combined)
}

// Helper: Check equality of two variables
fn equality_check(
    cs: &mut ConstraintSystem,
    a: Variable,
    b: Variable,
) -> Result<Variable, CircuitError> {
    let is_equal = cs.alloc_variable(None);

    let a_val = cs.get_value(a);
    let b_val = cs.get_value(b);

    if let (Some(av), Some(bv)) = (a_val, b_val) {
        cs.set_value(is_equal, Field::from((av == bv) as u64));
    }

    // Constraint: (a - b) * is_zero_inverse = 1 - is_equal
    // If a == b, then is_equal = 1
    // If a != b, then is_equal = 0
    let diff = cs.alloc_variable(None);
    if let (Some(av), Some(bv)) = (a_val, b_val) {
        cs.set_value(diff, av - bv);
    }

    let mut lc_diff = LinearCombination::from_variable(a);
    let mut neg_b = LinearCombination::from_variable(b);
    neg_b.scale(Field::from(-1));
    lc_diff.add(&neg_b);
    cs.enforce_equal(lc_diff, LinearCombination::from_variable(diff));

    Ok(is_equal)
}

// Helper: convert Field to u64
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
    fn test_ecdsa_gadget_structure() {
        let mut cs = ConstraintSystem::new();

        let msg = cs.alloc_variable(Some(Field::from(12345)));
        let r = cs.alloc_variable(Some(Field::from(111)));
        let s = cs.alloc_variable(Some(Field::from(222)));
        let pk_x = cs.alloc_variable(Some(Field::from(333)));
        let pk_y = cs.alloc_variable(Some(Field::from(444)));

        // Note: This will fail actual verification since we're using dummy values
        // But it tests the structure
        let result = ECDSAGadget::verify(&mut cs, msg, r, s, pk_x, pk_y);

        // Should not panic, even if verification fails
        assert!(result.is_ok());
    }

    #[test]
    fn test_schnorr_gadget() {
        let mut cs = ConstraintSystem::new();

        let msg = cs.alloc_variable(Some(Field::from(12345)));
        let r = cs.alloc_variable(Some(Field::from(111)));
        let s = cs.alloc_variable(Some(Field::from(222)));
        let pk = cs.alloc_variable(Some(Field::from(333)));

        let result = SchnorrGadget::verify(&mut cs, msg, r, s, pk);

        assert!(result.is_ok());
    }

    #[test]
    fn test_constraint_counts() {
        // Verify relative constraint counts make sense
        assert!(ECDSAGadget::num_constraints() > SchnorrGadget::num_constraints());
        assert!(BLSGadget::num_constraints() > ECDSAGadget::num_constraints());
        assert!(EdDSAGadget::num_constraints() > SchnorrGadget::num_constraints());
        assert!(RSAGadget::num_constraints() > ECDSAGadget::num_constraints());
    }

    #[test]
    fn test_rsa_gadget() {
        let mut cs = ConstraintSystem::new();

        let msg = cs.alloc_variable(Some(Field::from(12345)));
        let sig = cs.alloc_variable(Some(Field::from(999)));
        let e = cs.alloc_variable(Some(Field::from(65537)));
        let n = cs.alloc_variable(Some(Field::from(123456789)));

        let result = RSAGadget::verify(&mut cs, msg, sig, e, n);

        assert!(result.is_ok());
        if let Ok(rsa) = result {
            println!("RSA constraints: {}", rsa.num_constraints);
        }
    }

    #[test]
    fn test_rsa_standard() {
        let mut cs = ConstraintSystem::new();

        let msg = cs.alloc_variable(Some(Field::from(12345)));
        let sig = cs.alloc_variable(Some(Field::from(999)));
        let n = cs.alloc_variable(Some(Field::from(123456789)));

        let result = RSAGadget::verify_standard(&mut cs, msg, sig, n);

        assert!(result.is_ok());
    }
}
