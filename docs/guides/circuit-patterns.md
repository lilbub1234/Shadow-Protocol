# Circuit Design Patterns

A comprehensive guide to zero-knowledge circuit design patterns in Shroud Framework.

## Table of Contents

1. [Introduction](#introduction)
2. [Basic Patterns](#basic-patterns)
3. [Privacy Patterns](#privacy-patterns)
4. [Optimization Patterns](#optimization-patterns)
5. [Security Patterns](#security-patterns)
6. [Composition Patterns](#composition-patterns)

---

## Introduction

Zero-knowledge circuits require careful design to balance security, performance, and functionality. This guide presents battle-tested patterns for common scenarios.

### Circuit Design Principles

1. **Minimize Constraints**: Fewer constraints = faster proving
2. **Leverage Existing Gadgets**: Reuse well-tested components
3. **Think in Commitments**: Use hashing to compress data
4. **Nullifiers Prevent Reuse**: Unique nullifiers prevent double-spending/voting
5. **Range Checks Are Expensive**: Use sparingly and wisely

---

## Basic Patterns

### 1. **Hash-and-Check Pattern**

**Problem**: Prove knowledge of a secret without revealing it.

**Solution**: Hash the secret and prove the hash matches.

```rust
use shroud::prelude::*;

#[shroud::circuit]
pub struct HashCheck {
    #[private]
    secret: Field,

    #[public]
    hash: Field,
}

impl Circuit for HashCheck {
    fn constraints(&self) -> Result<()> {
        let computed_hash = poseidon_hash(&[self.secret]);
        assert_equal(computed_hash, self.hash)?;
        Ok(())
    }
}
```

**Use Cases**:
- Password verification
- Commitment schemes
- Preimage proofs

**Constraints**: ~100-150 (for Poseidon hash)

---

### 2. **Merkle Membership Pattern**

**Problem**: Prove an element is in a set without revealing which element.

**Solution**: Use a Merkle tree with zero-knowledge proof of inclusion.

```rust
#[shroud::circuit]
pub struct MerkleMembership {
    #[private]
    leaf: Field,

    #[private]
    path: Vec<Field>,

    #[private]
    indices: Vec<bool>,

    #[public]
    root: Field,
}

impl Circuit for MerkleMembership {
    fn constraints(&self) -> Result<()> {
        MerklePathGadget::verify(
            &mut self.cs,
            self.leaf,
            &self.path,
            &self.indices,
            self.root,
        )?;
        Ok(())
    }
}
```

**Use Cases**:
- Voter eligibility
- Whitelist membership
- Anonymous credentials

**Constraints**: ~150 * tree_depth

**Optimization**: Use shallow, wide trees for better performance.

---

### 3. **Range Proof Pattern**

**Problem**: Prove a value is within a range without revealing the exact value.

**Solution**: Binary decomposition with bit constraints.

```rust
#[shroud::circuit]
pub struct RangeProof {
    #[private]
    value: Field,

    #[public]
    min: u64,

    #[public]
    max: u64,
}

impl Circuit for RangeProof {
    fn constraints(&self) -> Result<()> {
        RangeCheckGadget::new(
            &mut self.cs,
            self.value,
            self.min,
            self.max,
        )?;
        Ok(())
    }
}
```

**Use Cases**:
- Age verification (age >= 18)
- Balance checks (balance >= amount)
- Bid validation (min_bid <= bid <= max_bid)

**Constraints**: ~2 * log₂(range_size)

**Optimization**: Minimize the range size to reduce constraints.

---

## Privacy Patterns

### 4. **Nullifier Pattern**

**Problem**: Prevent double-spending or double-voting while maintaining privacy.

**Solution**: Derive a unique nullifier from the secret and context.

```rust
#[shroud::circuit]
pub struct NullifierCircuit {
    #[private]
    secret: Field,

    #[private]
    context: Field, // e.g., vote_id, transaction_id

    #[public]
    nullifier: Field,
}

impl Circuit for NullifierCircuit {
    fn constraints(&self) -> Result<()> {
        let computed_nullifier = poseidon_hash(&[
            self.secret,
            self.context,
        ]);

        assert_equal(computed_nullifier, self.nullifier)?;
        Ok(())
    }
}
```

**Properties**:
- Unique per context
- Unlinkable to identity
- Deterministic (same inputs → same nullifier)

**Use Cases**:
- Anonymous voting (one vote per person)
- Privacy coins (prevent double-spend)
- Credential usage tracking

**Security Note**: Always include context to prevent nullifier reuse across applications.

---

### 5. **Selective Disclosure Pattern**

**Problem**: Reveal only specific attributes from a credential.

**Solution**: Separate commitments for different attributes.

```rust
#[shroud::circuit]
pub struct SelectiveDisclosure {
    #[private]
    full_credential: Vec<Field>, // [age, country, name, ...]

    #[private]
    reveal_mask: Vec<bool>, // [true, false, false, ...]

    #[public]
    revealed_attrs: Vec<Field>,

    #[public]
    credential_hash: Field,
}

impl Circuit for SelectiveDisclosure {
    fn constraints(&self) -> Result<()> {
        // Verify credential hash
        let computed_hash = poseidon_hash(&self.full_credential);
        assert_equal(computed_hash, self.credential_hash)?;

        // Conditionally reveal attributes
        for (i, &reveal) in self.reveal_mask.iter().enumerate() {
            if reveal {
                let revealed = ConditionalSelectGadget::new(
                    &mut self.cs,
                    reveal_var,
                    self.full_credential[i],
                    zero(),
                )?;

                assert_equal(revealed, self.revealed_attrs[i])?;
            }
        }

        Ok(())
    }
}
```

**Use Cases**:
- Age verification (reveal age >= 18, hide exact age)
- Location proof (reveal country, hide city)
- Partial identity disclosure

---

### 6. **Anonymous Set Membership Pattern**

**Problem**: Prove membership in a group without revealing identity.

**Solution**: Combine Merkle proof with nullifier.

```rust
#[shroud::circuit]
pub struct AnonymousSetMember {
    #[private]
    member_secret: Field,

    #[private]
    merkle_path: Vec<Field>,

    #[private]
    merkle_indices: Vec<bool>,

    #[public]
    set_root: Field,

    #[public]
    action_nullifier: Field, // Prevents action reuse

    #[public]
    action_context: Field, // e.g., "vote_2024"
}

impl Circuit for AnonymousSetMember {
    fn constraints(&self) -> Result<()> {
        // Prove membership
        let commitment = poseidon_hash(&[self.member_secret]);

        MerklePathGadget::verify(
            &mut self.cs,
            commitment,
            &self.merkle_path,
            &self.merkle_indices,
            self.set_root,
        )?;

        // Generate nullifier
        let computed_nullifier = poseidon_hash(&[
            self.member_secret,
            self.action_context,
        ]);

        assert_equal(computed_nullifier, self.action_nullifier)?;

        Ok(())
    }
}
```

**Use Cases**:
- DAO voting (prove membership, vote anonymously)
- Anonymous surveys
- Private airdrops

---

## Optimization Patterns

### 7. **Gadget Fusion Pattern**

**Problem**: Multiple small gadgets create overhead.

**Solution**: Combine related operations into a single gadget.

**Before** (Inefficient):
```rust
let hash1 = poseidon_hash(&[a]);
let hash2 = poseidon_hash(&[b]);
let combined = poseidon_hash(&[hash1, hash2]);
```

**After** (Optimized):
```rust
// Single gadget that hashes both and combines
let combined = poseidon_hash(&[a, b]); // Fewer rounds, more efficient
```

**Savings**: ~30% fewer constraints

---

### 8. **Lazy Evaluation Pattern**

**Problem**: Computing unnecessary intermediate values.

**Solution**: Only compute what's actually needed.

```rust
#[shroud::circuit]
pub struct LazyComputation {
    #[private]
    input: Field,

    #[private]
    condition: bool,

    #[public]
    output: Field,
}

impl Circuit for LazyComputation {
    fn constraints(&self) -> Result<()> {
        // Only compute expensive operation if needed
        let result = if self.condition {
            expensive_computation(self.input)?
        } else {
            cheap_computation(self.input)?
        };

        assert_equal(result, self.output)?;
        Ok(())
    }
}
```

---

### 9. **Batch Verification Pattern**

**Problem**: Verifying multiple proofs is expensive.

**Solution**: Aggregate proofs or use batch verification.

```rust
pub fn batch_verify_membership(
    cs: &mut ConstraintSystem,
    leaves: &[Variable],
    paths: &[Vec<Variable>],
    root: Variable,
) -> Result<Variable> {
    let mut all_valid = cs.alloc_variable(Some(Field::from(1)));

    for (leaf, path) in leaves.iter().zip(paths.iter()) {
        let valid = MerklePathGadget::verify(cs, *leaf, path, /* ... */)?;
        all_valid = and(cs, all_valid, valid.result())?;
    }

    Ok(all_valid)
}
```

**Savings**: O(n) → O(1) on-chain verification cost

---

## Security Patterns

### 10. **Input Validation Pattern**

**Problem**: Invalid inputs can break security assumptions.

**Solution**: Validate all inputs with constraints.

```rust
#[shroud::circuit]
pub struct SecureCircuit {
    #[private]
    amount: Field,

    #[private]
    signature_r: Field,

    #[private]
    signature_s: Field,

    #[public]
    public_key: Field,
}

impl Circuit for SecureCircuit {
    fn constraints(&self) -> Result<()> {
        // 1. Validate amount is in valid range
        RangeCheckGadget::new(&mut self.cs, self.amount, 0, MAX_AMOUNT)?;

        // 2. Validate signature components are not zero
        assert_not_zero(self.signature_r)?;
        assert_not_zero(self.signature_s)?;

        // 3. Validate public key is on curve
        validate_curve_point(self.public_key)?;

        // 4. Actual signature verification
        verify_signature(/* ... */)?;

        Ok(())
    }
}
```

**Always Validate**:
- Range bounds
- Non-zero values
- Curve points (for elliptic curve operations)
- Array lengths

---

### 11. **Commitment Binding Pattern**

**Problem**: Prevent commitment malleability.

**Solution**: Include all relevant context in the commitment.

```rust
pub fn secure_commitment(
    user_secret: Field,
    data: Field,
    timestamp: u64,
    context: &str,
) -> Field {
    poseidon_hash(&[
        user_secret,
        data,
        Field::from(timestamp),
        Field::from(hash_string(context)),
        // Include application-specific salt
        APPSALT,
    ])
}
```

**Security Properties**:
- Unique per application (via salt)
- Time-bound (via timestamp)
- Context-specific
- Non-malleable

---

### 12. **Double-Spend Prevention Pattern**

**Problem**: User might try to use the same proof multiple times.

**Solution**: Unique nullifiers + on-chain tracking.

```rust
// Circuit side
#[shroud::circuit]
pub struct SpendProof {
    #[private]
    coin_secret: Field,

    #[private]
    spend_context: Field,

    #[public]
    nullifier: Field,
}

// Contract side (Solidity)
contract AntiDoubleSpend {
    mapping(bytes32 => bool) public usedNullifiers;

    function spend(Proof memory proof, bytes32 nullifier) public {
        require(!usedNullifiers[nullifier], "Already spent");
        require(verify(proof), "Invalid proof");

        usedNullifiers[nullifier] = true;
        // ... perform action
    }
}
```

---

## Composition Patterns

### 13. **Circuit Modularization Pattern**

**Problem**: Large circuits are hard to maintain and optimize.

**Solution**: Break into reusable modules.

```rust
// Module 1: Authentication
pub fn authenticate_user(
    cs: &mut ConstraintSystem,
    user_secret: Variable,
    signature: (Variable, Variable),
) -> Result<Variable> {
    // Authentication logic
}

// Module 2: Authorization
pub fn check_permissions(
    cs: &mut ConstraintSystem,
    user_id: Variable,
    required_role: u32,
) -> Result<Variable> {
    // Authorization logic
}

// Composed circuit
#[shroud::circuit]
pub struct SecureAction {
    // ...
}

impl Circuit for SecureAction {
    fn constraints(&self) -> Result<()> {
        let authenticated = authenticate_user(/* ... */)?;
        let authorized = check_permissions(/* ... */)?;
        let can_act = and(&mut self.cs, authenticated, authorized)?;

        // Perform action if authorized
        // ...
    }
}
```

**Benefits**:
- Easier testing
- Reusable components
- Better optimization opportunities

---

### 14. **Recursive Verification Pattern**

**Problem**: Need to verify multiple proofs efficiently.

**Solution**: Verify proofs within proofs (recursive SNARKs).

```rust
#[shroud::circuit]
pub struct RecursiveVerifier {
    #[private]
    inner_proof: Proof,

    #[public]
    inner_public_inputs: Vec<Field>,

    #[public]
    aggregated_result: Field,
}

impl Circuit for RecursiveVerifier {
    fn constraints(&self) -> Result<()> {
        // Verify the inner proof within this circuit
        let valid = verify_proof_in_circuit(
            &mut self.cs,
            &self.inner_proof,
            &self.inner_public_inputs,
        )?;

        assert_equal(valid, Field::from(1))?;

        // Combine result with previous proofs
        // ...

        Ok(())
    }
}
```

**Use Cases**:
- Proof aggregation
- Blockchain light clients
- Incremental computation

---

## Anti-Patterns

### ❌ Don't: Expose Sensitive Data in Public Inputs

```rust
// BAD
#[public]
user_id: Field, // Reveals identity!

// GOOD
#[public]
user_commitment: Field, // Hash of user_id
```

### ❌ Don't: Reuse Nullifiers Across Contexts

```rust
// BAD
nullifier = hash(secret) // Same for all actions!

// GOOD
nullifier = hash(secret, action_context) // Unique per action
```

### ❌ Don't: Skip Input Validation

```rust
// BAD
fn transfer(amount: Field) {
    // Directly use amount without validation
}

// GOOD
fn transfer(amount: Field) {
    RangeCheck(amount, 0, MAX_AMOUNT)?;
    // Now safe to use
}
```

---

## Performance Tips

1. **Profile Before Optimizing**: Use `shroud bench` to find bottlenecks
2. **Cache Repeated Operations**: Reuse hash results when possible
3. **Choose the Right Hash**: Poseidon for ZK, SHA for interoperability
4. **Minimize Public Inputs**: Each public input adds verification cost
5. **Use Lookup Tables**: For repeated computations (e.g., range checks)

---

## Further Reading

- [ZK Circuit Best Practices](https://docs.shadeframework.io/best-practices)
- [Gadget Library Reference](https://docs.shadeframework.io/gadgets)
- [Performance Optimization Guide](https://docs.shadeframework.io/optimization)
- [Security Audit Checklist](https://docs.shadeframework.io/security-checklist)

---

**Questions?** Join our [Discord](https://discord.gg/shroud) or open a [GitHub Discussion](https://github.com/shroud-protocol/shroud/discussions).
