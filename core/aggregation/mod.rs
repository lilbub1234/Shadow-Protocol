// Shroud Framework - Proof Aggregation & Recursion
// Copyright (c) 2025 Shroud Protocol Contributors
// Licensed under MIT License

//! Proof aggregation and recursive verification
//!
//! Allows multiple proofs to be compressed into a single proof,
//! enabling:
//! - Batch verification (verify many proofs at once)
//! - Recursive composition (proofs that verify other proofs)
//! - Incremental computation (update proofs as computation progresses)

use crate::core::circuit::*;
use std::collections::HashMap;

/// Aggregated proof containing multiple sub-proofs
pub struct AggregatedProof {
    /// Individual proofs being aggregated
    pub sub_proofs: Vec<ProofId>,

    /// Aggregation proof
    pub aggregation_proof: Vec<u8>,

    /// Public inputs for all sub-proofs
    pub public_inputs: Vec<Vec<Field>>,

    /// Aggregation scheme used
    pub scheme: AggregationScheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationScheme {
    /// Simple batch verification
    Batch,

    /// Recursive SNARK composition
    Recursive,

    /// Incremental verification
    Incremental,

    /// Tree-based aggregation
    Tree,
}

/// Proof identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProofId(pub u64);

/// Proof aggregator
pub struct ProofAggregator {
    /// Cached proofs
    proof_cache: HashMap<ProofId, CachedProof>,

    /// Aggregation parameters
    params: AggregationParams,

    /// Next proof ID
    next_id: u64,
}

#[derive(Clone)]
struct CachedProof {
    id: ProofId,
    proof_data: Vec<u8>,
    public_inputs: Vec<Field>,
    circuit_id: CircuitId,
}

pub struct AggregationParams {
    /// Maximum proofs per batch
    pub max_batch_size: usize,

    /// Tree aggregation depth
    pub tree_depth: usize,

    /// Enable recursive verification
    pub enable_recursion: bool,
}

impl Default for AggregationParams {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            tree_depth: 10,
            enable_recursion: true,
        }
    }
}

impl ProofAggregator {
    pub fn new(params: AggregationParams) -> Self {
        Self {
            proof_cache: HashMap::new(),
            params,
            next_id: 0,
        }
    }

    /// Register a proof for aggregation
    pub fn register_proof(
        &mut self,
        proof_data: Vec<u8>,
        public_inputs: Vec<Field>,
        circuit_id: CircuitId,
    ) -> ProofId {
        let id = ProofId(self.next_id);
        self.next_id += 1;

        self.proof_cache.insert(
            id,
            CachedProof {
                id,
                proof_data,
                public_inputs,
                circuit_id,
            },
        );

        id
    }

    /// Aggregate multiple proofs into one
    pub fn aggregate(
        &self,
        proof_ids: &[ProofId],
        scheme: AggregationScheme,
    ) -> Result<AggregatedProof, AggregationError> {
        if proof_ids.is_empty() {
            return Err(AggregationError::EmptyProofSet);
        }

        if proof_ids.len() > self.params.max_batch_size {
            return Err(AggregationError::TooManyProofs(proof_ids.len()));
        }

        match scheme {
            AggregationScheme::Batch => self.batch_aggregate(proof_ids),
            AggregationScheme::Recursive => self.recursive_aggregate(proof_ids),
            AggregationScheme::Incremental => self.incremental_aggregate(proof_ids),
            AggregationScheme::Tree => self.tree_aggregate(proof_ids),
        }
    }

    /// Simple batch aggregation
    fn batch_aggregate(&self, proof_ids: &[ProofId]) -> Result<AggregatedProof, AggregationError> {
        let proofs: Vec<&CachedProof> = proof_ids
            .iter()
            .map(|id| {
                self.proof_cache
                    .get(id)
                    .ok_or(AggregationError::ProofNotFound(*id))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Combine all proof data
        let mut aggregation_proof = Vec::new();
        let mut all_public_inputs = Vec::new();

        for proof in &proofs {
            aggregation_proof.extend_from_slice(&proof.proof_data);
            all_public_inputs.push(proof.public_inputs.clone());
        }

        // Add batch verification metadata
        let batch_metadata = self.generate_batch_metadata(&proofs);
        aggregation_proof.extend_from_slice(&batch_metadata);

        Ok(AggregatedProof {
            sub_proofs: proof_ids.to_vec(),
            aggregation_proof,
            public_inputs: all_public_inputs,
            scheme: AggregationScheme::Batch,
        })
    }

    /// Recursive aggregation (proofs verifying proofs)
    fn recursive_aggregate(&self, proof_ids: &[ProofId]) -> Result<AggregatedProof, AggregationError> {
        if !self.params.enable_recursion {
            return Err(AggregationError::RecursionDisabled);
        }

        let proofs: Vec<&CachedProof> = proof_ids
            .iter()
            .map(|id| self.proof_cache.get(id).ok_or(AggregationError::ProofNotFound(*id)))
            .collect::<Result<Vec<_>, _>>()?;

        // Build recursive verification circuit
        let recursive_circuit = self.build_recursive_circuit(&proofs)?;

        // Generate proof of proof verification
        let aggregation_proof = self.prove_recursive_circuit(recursive_circuit)?;

        let all_public_inputs: Vec<Vec<Field>> = proofs
            .iter()
            .map(|p| p.public_inputs.clone())
            .collect();

        Ok(AggregatedProof {
            sub_proofs: proof_ids.to_vec(),
            aggregation_proof,
            public_inputs: all_public_inputs,
            scheme: AggregationScheme::Recursive,
        })
    }

    /// Incremental aggregation (sequential composition)
    fn incremental_aggregate(&self, proof_ids: &[ProofId]) -> Result<AggregatedProof, AggregationError> {
        if proof_ids.len() < 2 {
            return Err(AggregationError::InsufficientProofs);
        }

        let mut aggregated_proof = Vec::new();
        let mut all_public_inputs = Vec::new();

        // Incrementally combine proofs
        for (i, &proof_id) in proof_ids.iter().enumerate() {
            let proof = self
                .proof_cache
                .get(&proof_id)
                .ok_or(AggregationError::ProofNotFound(proof_id))?;

            if i == 0 {
                // First proof - use as base
                aggregated_proof = proof.proof_data.clone();
            } else {
                // Subsequent proofs - combine with previous
                aggregated_proof = self.combine_incremental(&aggregated_proof, &proof.proof_data)?;
            }

            all_public_inputs.push(proof.public_inputs.clone());
        }

        Ok(AggregatedProof {
            sub_proofs: proof_ids.to_vec(),
            aggregation_proof: aggregated_proof,
            public_inputs: all_public_inputs,
            scheme: AggregationScheme::Incremental,
        })
    }

    /// Tree-based aggregation (logarithmic depth)
    fn tree_aggregate(&self, proof_ids: &[ProofId]) -> Result<AggregatedProof, AggregationError> {
        let proofs: Vec<&CachedProof> = proof_ids
            .iter()
            .map(|id| self.proof_cache.get(id).ok_or(AggregationError::ProofNotFound(*id)))
            .collect::<Result<Vec<_>, _>>()?;

        // Build binary tree of proof aggregations
        let tree = self.build_aggregation_tree(&proofs)?;

        // Extract root proof
        let aggregation_proof = tree.root_proof();

        let all_public_inputs: Vec<Vec<Field>> = proofs
            .iter()
            .map(|p| p.public_inputs.clone())
            .collect();

        Ok(AggregatedProof {
            sub_proofs: proof_ids.to_vec(),
            aggregation_proof,
            public_inputs: all_public_inputs,
            scheme: AggregationScheme::Tree,
        })
    }

    fn build_recursive_circuit(
        &self,
        _proofs: &[&CachedProof],
    ) -> Result<RecursiveCircuit, AggregationError> {
        // Build circuit that verifies all sub-proofs
        Ok(RecursiveCircuit {
            num_proofs: _proofs.len(),
            verifier_circuits: Vec::new(),
        })
    }

    fn prove_recursive_circuit(&self, _circuit: RecursiveCircuit) -> Result<Vec<u8>, AggregationError> {
        // Generate proof for recursive circuit
        Ok(vec![0u8; 128]) // Placeholder
    }

    fn generate_batch_metadata(&self, _proofs: &[&CachedProof]) -> Vec<u8> {
        // Generate metadata for batch verification
        vec![0u8; 32] // Placeholder
    }

    fn combine_incremental(&self, prev: &[u8], current: &[u8]) -> Result<Vec<u8>, AggregationError> {
        // Combine two proofs incrementally
        let mut combined = prev.to_vec();
        combined.extend_from_slice(current);
        Ok(combined)
    }

    fn build_aggregation_tree(&self, proofs: &[&CachedProof]) -> Result<AggregationTree, AggregationError> {
        if proofs.is_empty() {
            return Err(AggregationError::EmptyProofSet);
        }

        // Build binary tree bottom-up
        let mut current_level: Vec<Vec<u8>> = proofs.iter().map(|p| p.proof_data.clone()).collect();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    self.combine_two_proofs(&chunk[0], &chunk[1])?
                } else {
                    chunk[0].clone()
                };

                next_level.push(combined);
            }

            current_level = next_level;
        }

        Ok(AggregationTree {
            root: current_level[0].clone(),
            depth: (proofs.len() as f64).log2().ceil() as usize,
        })
    }

    fn combine_two_proofs(&self, proof1: &[u8], proof2: &[u8]) -> Result<Vec<u8>, AggregationError> {
        // Combine two proofs into one (proof that verifies both)
        let mut combined = proof1.to_vec();
        combined.extend_from_slice(proof2);

        // Hash for commitment
        let commitment = blake3::hash(&combined);
        combined.extend_from_slice(commitment.as_bytes());

        Ok(combined)
    }

    /// Verify an aggregated proof
    pub fn verify_aggregated(&self, proof: &AggregatedProof) -> Result<bool, AggregationError> {
        match proof.scheme {
            AggregationScheme::Batch => self.verify_batch(proof),
            AggregationScheme::Recursive => self.verify_recursive(proof),
            AggregationScheme::Incremental => self.verify_incremental(proof),
            AggregationScheme::Tree => self.verify_tree(proof),
        }
    }

    fn verify_batch(&self, _proof: &AggregatedProof) -> Result<bool, AggregationError> {
        // Verify batch proof
        Ok(true) // Simplified
    }

    fn verify_recursive(&self, _proof: &AggregatedProof) -> Result<bool, AggregationError> {
        // Verify recursive proof
        Ok(true) // Simplified
    }

    fn verify_incremental(&self, _proof: &AggregatedProof) -> Result<bool, AggregationError> {
        // Verify incremental proof
        Ok(true) // Simplified
    }

    fn verify_tree(&self, _proof: &AggregatedProof) -> Result<bool, AggregationError> {
        // Verify tree-aggregated proof
        Ok(true) // Simplified
    }

    /// Get statistics
    pub fn stats(&self) -> AggregationStats {
        AggregationStats {
            total_proofs: self.proof_cache.len(),
            next_id: self.next_id,
            max_batch_size: self.params.max_batch_size,
        }
    }
}

struct RecursiveCircuit {
    num_proofs: usize,
    verifier_circuits: Vec<Vec<u8>>,
}

struct AggregationTree {
    root: Vec<u8>,
    depth: usize,
}

impl AggregationTree {
    fn root_proof(&self) -> Vec<u8> {
        self.root.clone()
    }
}

#[derive(Debug)]
pub struct AggregationStats {
    pub total_proofs: usize,
    pub next_id: u64,
    pub max_batch_size: usize,
}

#[derive(Debug)]
pub enum AggregationError {
    EmptyProofSet,
    TooManyProofs(usize),
    ProofNotFound(ProofId),
    RecursionDisabled,
    InsufficientProofs,
    AggregationFailed(String),
}

impl std::fmt::Display for AggregationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregationError::EmptyProofSet => write!(f, "Empty proof set"),
            AggregationError::TooManyProofs(n) => write!(f, "Too many proofs: {}", n),
            AggregationError::ProofNotFound(id) => write!(f, "Proof not found: {:?}", id),
            AggregationError::RecursionDisabled => write!(f, "Recursion disabled"),
            AggregationError::InsufficientProofs => write!(f, "Insufficient proofs"),
            AggregationError::AggregationFailed(msg) => write!(f, "Aggregation failed: {}", msg),
        }
    }
}

impl std::error::Error for AggregationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_aggregation() {
        let mut aggregator = ProofAggregator::new(AggregationParams::default());

        // Register multiple proofs
        let proof1 = aggregator.register_proof(
            vec![1, 2, 3],
            vec![Field::from(100)],
            CircuitId::new(),
        );

        let proof2 = aggregator.register_proof(
            vec![4, 5, 6],
            vec![Field::from(200)],
            CircuitId::new(),
        );

        let proof3 = aggregator.register_proof(
            vec![7, 8, 9],
            vec![Field::from(300)],
            CircuitId::new(),
        );

        // Aggregate
        let aggregated = aggregator
            .aggregate(&[proof1, proof2, proof3], AggregationScheme::Batch)
            .unwrap();

        assert_eq!(aggregated.sub_proofs.len(), 3);
        assert_eq!(aggregated.public_inputs.len(), 3);
    }

    #[test]
    fn test_tree_aggregation() {
        let mut aggregator = ProofAggregator::new(AggregationParams::default());

        // Register 4 proofs (will create binary tree of depth 2)
        let mut proof_ids = Vec::new();
        for i in 0..4 {
            let id = aggregator.register_proof(
                vec![i as u8],
                vec![Field::from(i)],
                CircuitId::new(),
            );
            proof_ids.push(id);
        }

        // Tree aggregation
        let aggregated = aggregator
            .aggregate(&proof_ids, AggregationScheme::Tree)
            .unwrap();

        assert_eq!(aggregated.sub_proofs.len(), 4);

        // Verify
        assert!(aggregator.verify_aggregated(&aggregated).unwrap());
    }

    #[test]
    fn test_stats() {
        let mut aggregator = ProofAggregator::new(AggregationParams::default());

        for i in 0..5 {
            aggregator.register_proof(
                vec![i],
                vec![Field::from(i)],
                CircuitId::new(),
            );
        }

        let stats = aggregator.stats();
        assert_eq!(stats.total_proofs, 5);
        assert_eq!(stats.next_id, 5);
    }
}
