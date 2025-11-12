// Hello Shroud - Your First Zero-Knowledge Circuit
// This example demonstrates a simple circuit that proves knowledge of a secret
// without revealing it.

use shroud::prelude::*;

/// A simple circuit that proves "I know the secret to this hash"
#[shroud::circuit]
pub struct HelloShroud {
    /// Private input: the secret (never revealed)
    #[private]
    secret: Field,

    /// Public output: hash of the secret (publicly visible)
    #[public]
    hash: Field,
}

impl Circuit for HelloShroud {
    fn constraints(&self) -> Result<()> {
        // Constraint: hash must equal poseidon_hash(secret)
        let computed_hash = poseidon_hash(&[self.secret]);
        self.assert_equal(computed_hash, self.hash)?;

        Ok(())
    }
}

fn main() {
    println!("🔐 Hello Shroud - Your First Zero-Knowledge Circuit\n");

    // Step 1: Generate a secret
    let secret = Field::from(42);
    println!("Step 1: Secret generated (never reveal this!)");
    println!("  Secret: {}", secret);

    // Step 2: Compute the hash
    let hash = poseidon_hash(&[secret]);
    println!("\nStep 2: Hash computed");
    println!("  Hash: {}", hash);

    // Step 3: Create the circuit
    let circuit = HelloShroud { secret, hash };
    println!("\nStep 3: Circuit created");

    // Step 4: Generate a proof
    println!("\nStep 4: Generating proof...");
    let start = std::time::Instant::now();
    let proof = circuit.prove().expect("Failed to generate proof");
    let duration = start.elapsed();
    println!("  ✓ Proof generated in {:?}", duration);
    println!("  Proof size: {} bytes", proof.bytes().len());

    // Step 5: Verify the proof
    println!("\nStep 5: Verifying proof...");
    let start = std::time::Instant::now();
    let valid = proof.verify();
    let duration = start.elapsed();
    println!("  ✓ Proof verified in {:?}", duration);
    println!("  Valid: {}", valid);

    // Explanation
    println!("\n📚 What just happened?");
    println!("  1. We created a secret (42)");
    println!("  2. We computed its hash");
    println!("  3. We generated a zero-knowledge proof that proves:");
    println!("     'I know a secret whose hash is {}'", hash);
    println!("  4. The proof reveals NOTHING about the secret itself!");
    println!("  5. Anyone can verify the proof without learning the secret");

    println!("\n✨ Welcome to zero-knowledge privacy with Shroud Framework!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_secret() {
        let secret = Field::from(12345);
        let hash = poseidon_hash(&[secret]);

        let circuit = HelloShroud { secret, hash };
        let proof = circuit.prove().unwrap();

        assert!(proof.verify());
    }

    #[test]
    fn test_invalid_secret() {
        let secret = Field::from(12345);
        let wrong_hash = Field::from(99999);

        let circuit = HelloShroud {
            secret,
            hash: wrong_hash,
        };

        // Should fail because hash doesn't match
        assert!(circuit.prove().is_err());
    }

    #[test]
    fn test_multiple_secrets() {
        // Different secrets should produce different hashes
        let secret1 = Field::from(1);
        let secret2 = Field::from(2);

        let hash1 = poseidon_hash(&[secret1]);
        let hash2 = poseidon_hash(&[secret2]);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_deterministic() {
        // Same secret should always produce same hash
        let secret = Field::from(42);

        let hash1 = poseidon_hash(&[secret]);
        let hash2 = poseidon_hash(&[secret]);

        assert_eq!(hash1, hash2);
    }
}
