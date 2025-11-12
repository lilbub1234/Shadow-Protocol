// Zero-Knowledge Credential System
// Proves attributes without revealing identity

use shroud::prelude::*;

/// Credential circuit
///
/// Proves:
/// 1. User has a valid credential signed by issuer
/// 2. Credential attributes satisfy requirements
/// 3. Credential is not revoked
/// WITHOUT revealing user identity or full credential data
#[shroud::circuit]
pub struct CredentialCircuit {
    // Private inputs
    #[private]
    user_secret: Field,              // User's secret key

    #[private]
    credential_data: Vec<Field>,     // Credential attributes

    #[private]
    issuer_signature_r: Field,       // Issuer's signature (r, s)
    #[private]
    issuer_signature_s: Field,

    #[private]
    merkle_path: Vec<Field>,         // Path in revocation tree
    #[private]
    merkle_indices: Vec<bool>,

    // Public inputs
    #[public]
    issuer_public_key: Field,        // Issuer's public key

    #[public]
    nullifier: Field,                // Unique nullifier (prevents reuse)

    #[public]
    requirement_hash: Field,         // Hash of requirements

    #[public]
    revocation_root: Field,          // Merkle root of non-revoked credentials

    #[public]
    timestamp: u64,                  // Current timestamp
}

impl Circuit for CredentialCircuit {
    fn constraints(&self) -> Result<()> {
        let mut cs = ConstraintSystem::new();

        // 1. Compute credential commitment
        let credential_commitment = poseidon_hash(&[
            self.user_secret,
            self.credential_data[0], // e.g., age
            self.credential_data[1], // e.g., country
            self.credential_data[2], // e.g., expiry
        ]);

        let commitment_var = cs.alloc_private_input(credential_commitment);

        // 2. Verify issuer signature on credential
        let sig_gadget = SchnorrGadget::verify(
            &mut cs,
            commitment_var,
            cs.alloc_private_input(self.issuer_signature_r),
            cs.alloc_private_input(self.issuer_signature_s),
            cs.alloc_public_input(self.issuer_public_key),
        )?;

        // Ensure signature is valid
        let one = cs.alloc_variable(Some(Field::from(1)));
        EqualityGadget::new(&mut cs, sig_gadget.result(), one)?;

        // 3. Verify credential is not revoked
        let merkle_gadget = MerklePathGadget::verify(
            &mut cs,
            commitment_var,
            &self.merkle_path_vars(),
            &self.merkle_indices,
            cs.alloc_public_input(self.revocation_root),
        )?;

        // 4. Verify credential attributes meet requirements
        // Example: Age >= 18
        let age_var = cs.alloc_private_input(self.credential_data[0]);
        RangeCheckGadget::new(&mut cs, age_var, 18, 150)?;

        // Example: Credential not expired
        let expiry_var = cs.alloc_private_input(self.credential_data[2]);
        let timestamp_var = cs.alloc_public_input(Field::from(self.timestamp));
        let not_expired = LessThanGadget::new(&mut cs, timestamp_var, expiry_var, 64)?;
        EqualityGadget::new(&mut cs, not_expired.result(), one)?;

        // 5. Verify requirement hash
        let requirements_hash = poseidon_hash(&[
            Field::from(18), // Min age
            // Other requirements...
        ]);
        let req_hash_var = cs.alloc_public_input(self.requirement_hash);
        EqualityGadget::new(&mut cs, cs.alloc_private_input(requirements_hash), req_hash_var)?;

        // 6. Compute nullifier (prevents credential reuse)
        let computed_nullifier = poseidon_hash(&[
            self.user_secret,
            self.requirement_hash,
            Field::from(self.timestamp / 86400), // Day granularity
        ]);
        let nullifier_var = cs.alloc_public_input(self.nullifier);
        EqualityGadget::new(&mut cs, cs.alloc_private_input(computed_nullifier), nullifier_var)?;

        // Check all constraints satisfied
        if !cs.is_satisfied() {
            return Err(ShroudError::ConstraintNotSatisfied(
                "Credential constraints not satisfied".to_string()
            ));
        }

        Ok(())
    }

    fn num_public_inputs(&self) -> usize {
        5 // issuer_pk, nullifier, req_hash, revocation_root, timestamp
    }

    fn num_private_inputs(&self) -> usize {
        5 + self.credential_data.len() + self.merkle_path.len()
    }

    fn id(&self) -> CircuitId {
        CircuitId::new()
    }
}

impl CredentialCircuit {
    fn merkle_path_vars(&self) -> Vec<Variable> {
        self.merkle_path.iter()
            .enumerate()
            .map(|(i, _)| Variable(i + 20))
            .collect()
    }
}

/// Credential system
pub struct CredentialSystem {
    /// Issuer's keypair
    pub issuer_secret_key: Field,
    pub issuer_public_key: Field,

    /// Issued credentials
    pub issued_credentials: Vec<Credential>,

    /// Revocation list (Merkle tree of non-revoked credentials)
    pub revocation_tree: MerkleTree,

    /// Used nullifiers (prevents reuse)
    pub used_nullifiers: HashSet<Field>,
}

/// Credential data structure
#[derive(Debug, Clone)]
pub struct Credential {
    pub user_secret: Field,
    pub attributes: CredentialAttributes,
    pub signature: (Field, Field),
    pub commitment: Field,
    pub issued_at: u64,
}

#[derive(Debug, Clone)]
pub struct CredentialAttributes {
    pub age: u32,
    pub country: String,
    pub expiry: u64,
    // Add more attributes as needed
}

impl Credential {
    pub fn new(
        user_secret: Field,
        attributes: CredentialAttributes,
        issuer_sk: Field,
        timestamp: u64,
    ) -> Self {
        // Compute credential commitment
        let commitment = poseidon_hash(&[
            user_secret,
            Field::from(attributes.age),
            Field::from(hash_string(&attributes.country)),
            Field::from(attributes.expiry),
        ]);

        // Sign commitment
        let signature = schnorr_sign(commitment, issuer_sk);

        Self {
            user_secret,
            attributes,
            signature,
            commitment,
            issued_at: timestamp,
        }
    }

    pub fn to_circuit_inputs(&self) -> Vec<Field> {
        vec![
            Field::from(self.attributes.age),
            Field::from(hash_string(&self.attributes.country)),
            Field::from(self.attributes.expiry),
        ]
    }
}

impl CredentialSystem {
    pub fn new() -> Self {
        let issuer_secret_key = Field::from(rand::random::<u64>());
        let issuer_public_key = compute_public_key(issuer_secret_key);

        Self {
            issuer_secret_key,
            issuer_public_key,
            issued_credentials: Vec::new(),
            revocation_tree: MerkleTree::new(&[]),
            used_nullifiers: HashSet::new(),
        }
    }

    /// Issue a new credential
    pub fn issue_credential(
        &mut self,
        user_secret: Field,
        attributes: CredentialAttributes,
    ) -> Credential {
        let timestamp = current_timestamp();
        let credential = Credential::new(
            user_secret,
            attributes,
            self.issuer_secret_key,
            timestamp,
        );

        self.issued_credentials.push(credential.clone());

        // Update revocation tree
        let commitments: Vec<Field> = self.issued_credentials
            .iter()
            .map(|c| c.commitment)
            .collect();
        self.revocation_tree = MerkleTree::new(&commitments);

        credential
    }

    /// Revoke a credential
    pub fn revoke_credential(&mut self, commitment: Field) {
        self.issued_credentials.retain(|c| c.commitment != commitment);

        // Update revocation tree
        let commitments: Vec<Field> = self.issued_credentials
            .iter()
            .map(|c| c.commitment)
            .collect();
        self.revocation_tree = MerkleTree::new(&commitments);
    }

    /// Verify a credential proof
    pub fn verify_credential(
        &mut self,
        proof: Proof,
        nullifier: Field,
        requirement_hash: Field,
    ) -> Result<(), CredentialError> {
        // Check nullifier hasn't been used
        if self.used_nullifiers.contains(&nullifier) {
            return Err(CredentialError::NullifierAlreadyUsed);
        }

        // Verify proof
        let public_inputs = vec![
            self.issuer_public_key,
            nullifier,
            requirement_hash,
            self.revocation_tree.root(),
            Field::from(current_timestamp()),
        ];

        if !proof.verify(&public_inputs) {
            return Err(CredentialError::InvalidProof);
        }

        // Mark nullifier as used
        self.used_nullifiers.insert(nullifier);

        Ok(())
    }

    /// Check credential status
    pub fn is_revoked(&self, commitment: Field) -> bool {
        !self.issued_credentials.iter().any(|c| c.commitment == commitment)
    }

    /// Get statistics
    pub fn stats(&self) -> CredentialStats {
        CredentialStats {
            total_issued: self.issued_credentials.len(),
            total_revoked: 0, // Would need to track separately
            verifications: self.used_nullifiers.len(),
        }
    }
}

#[derive(Debug)]
pub struct CredentialStats {
    pub total_issued: usize,
    pub total_revoked: usize,
    pub verifications: usize,
}

#[derive(Debug)]
pub enum CredentialError {
    NullifierAlreadyUsed,
    InvalidProof,
    CredentialExpired,
    CredentialRevoked,
    InsufficientAttributes,
}

// Helper functions
fn hash_string(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn schnorr_sign(message: Field, secret_key: Field) -> (Field, Field) {
    // Simplified Schnorr signature
    // Real implementation would use proper elliptic curve operations
    let r = Field::from(rand::random::<u64>());
    let s = message * secret_key + r;
    (r, s)
}

fn compute_public_key(secret_key: Field) -> Field {
    // Simplified public key computation
    // Real implementation would use elliptic curve scalar multiplication
    secret_key * Field::from(7) // Base point multiplied by SK
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_issuance() {
        let mut system = CredentialSystem::new();

        let user_secret = Field::from(12345);
        let attributes = CredentialAttributes {
            age: 25,
            country: "US".to_string(),
            expiry: current_timestamp() + 365 * 86400, // 1 year
        };

        let credential = system.issue_credential(user_secret, attributes);

        assert_eq!(system.issued_credentials.len(), 1);
        assert!(!system.is_revoked(credential.commitment));
    }

    #[test]
    fn test_credential_revocation() {
        let mut system = CredentialSystem::new();

        let user_secret = Field::from(12345);
        let attributes = CredentialAttributes {
            age: 25,
            country: "US".to_string(),
            expiry: current_timestamp() + 365 * 86400,
        };

        let credential = system.issue_credential(user_secret, attributes);
        system.revoke_credential(credential.commitment);

        assert!(system.is_revoked(credential.commitment));
    }

    #[test]
    fn test_credential_circuit() {
        let system = CredentialSystem::new();

        let user_secret = Field::from(12345);
        let attributes = vec![
            Field::from(25),  // age
            Field::from(840), // country code
            Field::from(current_timestamp() + 365 * 86400), // expiry
        ];

        let (r, s) = schnorr_sign(poseidon_hash(&attributes), system.issuer_secret_key);

        let circuit = CredentialCircuit {
            user_secret,
            credential_data: attributes,
            issuer_signature_r: r,
            issuer_signature_s: s,
            merkle_path: vec![],
            merkle_indices: vec![],
            issuer_public_key: system.issuer_public_key,
            nullifier: Field::from(999),
            requirement_hash: Field::from(111),
            revocation_root: system.revocation_tree.root(),
            timestamp: current_timestamp(),
        };

        // Test circuit can be constructed
        assert!(circuit.num_public_inputs() == 5);
    }

    #[test]
    fn test_stats() {
        let mut system = CredentialSystem::new();

        for i in 0..5 {
            let user_secret = Field::from(1000 + i);
            let attributes = CredentialAttributes {
                age: 20 + i as u32,
                country: "US".to_string(),
                expiry: current_timestamp() + 365 * 86400,
            };
            system.issue_credential(user_secret, attributes);
        }

        let stats = system.stats();
        assert_eq!(stats.total_issued, 5);
    }
}
