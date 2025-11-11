// Anonymous Voting Circuit
// Proves voter eligibility without revealing identity

use shroud::prelude::*;

/// Anonymous voting circuit
///
/// Proves:
/// 1. Voter knows a secret corresponding to an eligible voter (merkle proof)
/// 2. Voter hasn't voted before (nullifier not used)
/// 3. Vote is valid (within range of candidates)
#[shroud::circuit]
pub struct AnonymousVote {
    // Private inputs
    #[private]
    voter_secret: Field,           // Secret key of voter

    #[private]
    nullifier_secret: Field,       // Secret for nullifier generation

    #[private]
    merkle_path: Vec<Field>,       // Merkle proof of voter eligibility

    #[private]
    merkle_indices: Vec<bool>,     // Path indices (left/right)

    #[private]
    vote_choice: u32,              // Which candidate (0, 1, 2, ...)

    // Public inputs
    #[public]
    merkle_root: Field,            // Root of eligible voters tree

    #[public]
    nullifier: Field,              // Prevents double-voting

    #[public]
    vote_commitment: Field,        // Encrypted vote

    #[public]
    num_candidates: u32,           // Number of candidates
}

impl Circuit for AnonymousVote {
    fn constraints(&self) -> Result<()> {
        let mut cs = ConstraintSystem::new();

        // 1. Compute voter commitment from secret
        let voter_commitment = poseidon_hash(&[self.voter_secret]);
        let voter_commitment_var = cs.alloc_private_input(voter_commitment);

        // 2. Verify voter is in the eligible voters tree
        let merkle_gadget = MerklePathGadget::verify(
            &mut cs,
            voter_commitment_var,
            &self.merkle_path_vars(),
            &self.merkle_indices,
            cs.alloc_public_input(self.merkle_root),
        )?;

        // 3. Verify nullifier derivation (prevents double-voting)
        let computed_nullifier = poseidon_hash(&[
            self.voter_secret,
            self.nullifier_secret,
        ]);
        let nullifier_var = cs.alloc_public_input(self.nullifier);

        EqualityGadget::new(&mut cs,
            cs.alloc_private_input(computed_nullifier),
            nullifier_var
        )?;

        // 4. Verify vote is valid (in range of candidates)
        let vote_var = cs.alloc_private_input(Field::from(self.vote_choice));
        RangeCheckGadget::new(
            &mut cs,
            vote_var,
            0,
            self.num_candidates as u64 - 1,
        )?;

        // 5. Verify vote commitment
        let computed_commitment = poseidon_hash(&[
            Field::from(self.vote_choice),
            self.voter_secret,
        ]);
        let commitment_var = cs.alloc_public_input(self.vote_commitment);

        EqualityGadget::new(&mut cs,
            cs.alloc_private_input(computed_commitment),
            commitment_var
        )?;

        // Check all constraints satisfied
        if !cs.is_satisfied() {
            return Err(ShadeError::ConstraintNotSatisfied(
                "Voting constraints not satisfied".to_string()
            ));
        }

        Ok(())
    }

    fn num_public_inputs(&self) -> usize {
        3 // merkle_root, nullifier, vote_commitment
    }

    fn num_private_inputs(&self) -> usize {
        4 + self.merkle_path.len() // secrets + vote + path
    }

    fn id(&self) -> CircuitId {
        CircuitId::new()
    }
}

impl AnonymousVote {
    fn merkle_path_vars(&self) -> Vec<Variable> {
        // Convert merkle path to variables (simplified)
        self.merkle_path.iter()
            .enumerate()
            .map(|(i, _)| Variable(i + 10))
            .collect()
    }
}

/// Voting system state
pub struct VotingSystem {
    /// Eligible voters (commitments)
    pub eligible_voters: Vec<Field>,

    /// Merkle tree of eligible voters
    pub voter_tree: MerkleTree,

    /// Used nullifiers (prevents double-voting)
    pub used_nullifiers: HashSet<Field>,

    /// Vote tallies (encrypted)
    pub votes: Vec<Field>,

    /// Number of candidates
    pub num_candidates: u32,
}

impl VotingSystem {
    pub fn new(eligible_voters: Vec<Field>, num_candidates: u32) -> Self {
        let voter_tree = MerkleTree::new(&eligible_voters);

        Self {
            eligible_voters,
            voter_tree,
            used_nullifiers: HashSet::new(),
            votes: Vec::new(),
            num_candidates,
        }
    }

    /// Cast a vote with zero-knowledge proof
    pub fn cast_vote(
        &mut self,
        proof: Proof,
        nullifier: Field,
        vote_commitment: Field,
    ) -> Result<(), VotingError> {
        // 1. Check nullifier hasn't been used
        if self.used_nullifiers.contains(&nullifier) {
            return Err(VotingError::DoubleVote);
        }

        // 2. Verify proof
        let public_inputs = vec![
            self.voter_tree.root(),
            nullifier,
            vote_commitment,
        ];

        if !proof.verify(&public_inputs) {
            return Err(VotingError::InvalidProof);
        }

        // 3. Record vote
        self.used_nullifiers.insert(nullifier);
        self.votes.push(vote_commitment);

        Ok(())
    }

    /// Tally votes (in this simplified version, reveals results)
    /// Real implementation would use homomorphic encryption
    pub fn tally_votes(&self) -> Vec<u32> {
        let mut tallies = vec![0; self.num_candidates as usize];

        // In real system, votes would be decrypted by threshold decryption
        // For now, simplified

        tallies
    }

    /// Check if a voter has voted (by checking nullifier)
    pub fn has_voted(&self, nullifier: Field) -> bool {
        self.used_nullifiers.contains(&nullifier)
    }

    /// Get total votes cast
    pub fn total_votes(&self) -> usize {
        self.votes.len()
    }
}

/// Merkle tree for voter eligibility
pub struct MerkleTree {
    leaves: Vec<Field>,
    root: Field,
}

impl MerkleTree {
    pub fn new(leaves: &[Field]) -> Self {
        let root = Self::compute_root(leaves);
        Self {
            leaves: leaves.to_vec(),
            root,
        }
    }

    pub fn root(&self) -> Field {
        self.root
    }

    pub fn prove(&self, leaf_index: usize) -> (Vec<Field>, Vec<bool>) {
        // Generate merkle proof for leaf at index
        let mut path = Vec::new();
        let mut indices = Vec::new();
        let mut index = leaf_index;
        let mut level = self.leaves.clone();

        while level.len() > 1 {
            let is_right = index % 2 == 1;
            let sibling_index = if is_right { index - 1 } else { index + 1 };

            if sibling_index < level.len() {
                path.push(level[sibling_index]);
                indices.push(is_right);
            }

            // Move to next level
            let mut next_level = Vec::new();
            for i in (0..level.len()).step_by(2) {
                let left = level[i];
                let right = if i + 1 < level.len() {
                    level[i + 1]
                } else {
                    level[i]
                };
                next_level.push(poseidon_hash(&[left, right]));
            }

            level = next_level;
            index /= 2;
        }

        (path, indices)
    }

    fn compute_root(leaves: &[Field]) -> Field {
        if leaves.is_empty() {
            return Field::from(0);
        }

        let mut level = leaves.to_vec();

        while level.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..level.len()).step_by(2) {
                let left = level[i];
                let right = if i + 1 < level.len() {
                    level[i + 1]
                } else {
                    level[i]
                };
                next_level.push(poseidon_hash(&[left, right]));
            }
            level = next_level;
        }

        level[0]
    }
}

#[derive(Debug)]
pub enum VotingError {
    DoubleVote,
    InvalidProof,
    InvalidVote,
}

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymous_vote_circuit() {
        // Setup: 3 eligible voters
        let voter1_secret = Field::from(123);
        let voter2_secret = Field::from(456);
        let voter3_secret = Field::from(789);

        let commitments = vec![
            poseidon_hash(&[voter1_secret]),
            poseidon_hash(&[voter2_secret]),
            poseidon_hash(&[voter3_secret]),
        ];

        let tree = MerkleTree::new(&commitments);
        let (path, indices) = tree.prove(0); // Voter 1 proves eligibility

        // Voter 1 votes for candidate 1
        let nullifier_secret = Field::from(999);
        let nullifier = poseidon_hash(&[voter1_secret, nullifier_secret]);
        let vote_choice = 1;
        let vote_commitment = poseidon_hash(&[Field::from(vote_choice), voter1_secret]);

        let circuit = AnonymousVote {
            voter_secret: voter1_secret,
            nullifier_secret,
            merkle_path: path,
            merkle_indices: indices,
            vote_choice,
            merkle_root: tree.root(),
            nullifier,
            vote_commitment,
            num_candidates: 3,
        };

        // Generate proof
        let proof = circuit.prove().unwrap();

        // Verify proof
        assert!(proof.verify());
    }

    #[test]
    fn test_voting_system() {
        // Setup voting system
        let voter_secrets = vec![
            Field::from(111),
            Field::from(222),
            Field::from(333),
        ];

        let commitments: Vec<Field> = voter_secrets.iter()
            .map(|s| poseidon_hash(&[*s]))
            .collect();

        let mut system = VotingSystem::new(commitments, 3);

        // Voter 1 casts vote
        let voter1_secret = voter_secrets[0];
        let nullifier_secret = Field::from(999);
        let nullifier = poseidon_hash(&[voter1_secret, nullifier_secret]);
        let vote_commitment = poseidon_hash(&[Field::from(1), voter1_secret]);

        let (path, indices) = system.voter_tree.prove(0);

        let circuit = AnonymousVote {
            voter_secret: voter1_secret,
            nullifier_secret,
            merkle_path: path,
            merkle_indices: indices,
            vote_choice: 1,
            merkle_root: system.voter_tree.root(),
            nullifier,
            vote_commitment,
            num_candidates: 3,
        };

        let proof = circuit.prove().unwrap();

        // Cast vote
        system.cast_vote(proof, nullifier, vote_commitment).unwrap();

        assert_eq!(system.total_votes(), 1);
        assert!(system.has_voted(nullifier));

        // Try to vote again with same nullifier (should fail)
        let result = system.cast_vote(proof, nullifier, vote_commitment);
        assert!(matches!(result, Err(VotingError::DoubleVote)));
    }

    #[test]
    fn test_merkle_tree() {
        let leaves = vec![
            Field::from(1),
            Field::from(2),
            Field::from(3),
            Field::from(4),
        ];

        let tree = MerkleTree::new(&leaves);
        let root = tree.root();

        // Verify proofs for all leaves
        for i in 0..leaves.len() {
            let (path, indices) = tree.prove(i);

            // Recompute root from path
            let mut current = leaves[i];
            for (sibling, is_right) in path.iter().zip(indices.iter()) {
                let (left, right) = if *is_right {
                    (*sibling, current)
                } else {
                    (current, *sibling)
                };
                current = poseidon_hash(&[left, right]);
            }

            assert_eq!(current, root);
        }
    }
}
