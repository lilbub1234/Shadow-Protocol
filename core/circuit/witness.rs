// Shroud Framework - Witness Generation
// Copyright (c) 2025 Shroud Protocol Contributors
// Licensed under MIT License

//! Witness generation and management

use super::*;
use std::collections::HashMap;

/// Witness contains all private inputs to a circuit
#[derive(Debug, Clone)]
pub struct Witness {
    /// Private variable assignments
    pub assignments: HashMap<String, Field>,
}

impl Witness {
    /// Create a new empty witness
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
        }
    }

    /// Add a private input
    pub fn add(&mut self, name: impl Into<String>, value: Field) {
        self.assignments.insert(name.into(), value);
    }

    /// Get a private input
    pub fn get(&self, name: &str) -> Option<Field> {
        self.assignments.get(name).copied()
    }

    /// Check if witness contains a value
    pub fn contains(&self, name: &str) -> bool {
        self.assignments.contains_key(name)
    }

    /// Get all variable names
    pub fn variables(&self) -> Vec<String> {
        self.assignments.keys().cloned().collect()
    }

    /// Number of private inputs
    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    /// Check if witness is empty
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    /// Merge another witness into this one
    pub fn merge(&mut self, other: Witness) {
        self.assignments.extend(other.assignments);
    }

    /// Create from key-value pairs
    pub fn from_pairs(pairs: Vec<(&str, Field)>) -> Self {
        let mut witness = Self::new();
        for (name, value) in pairs {
            witness.add(name, value);
        }
        witness
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> String {
        use serde_json::json;

        let obj: HashMap<String, String> = self.assignments.iter()
            .map(|(k, v)| (k.clone(), format!("{:?}", v)))
            .collect();

        json!(obj).to_string()
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, CircuitError> {
        // Simplified - would need proper field element parsing
        Ok(Self::new())
    }
}

/// Witness builder for constructing witnesses programmatically
pub struct WitnessBuilder {
    witness: Witness,
}

impl WitnessBuilder {
    pub fn new() -> Self {
        Self {
            witness: Witness::new(),
        }
    }

    /// Add a field element
    pub fn add_field(mut self, name: &str, value: Field) -> Self {
        self.witness.add(name, value);
        self
    }

    /// Add a u64 value
    pub fn add_u64(mut self, name: &str, value: u64) -> Self {
        self.witness.add(name, Field::from(value));
        self
    }

    /// Add a boolean value
    pub fn add_bool(mut self, name: &str, value: bool) -> Self {
        self.witness.add(name, Field::from(value as u64));
        self
    }

    /// Add a byte array (as multiple field elements)
    pub fn add_bytes(mut self, name_prefix: &str, bytes: &[u8]) -> Self {
        for (i, byte) in bytes.iter().enumerate() {
            let name = format!("{}_{}", name_prefix, i);
            self.witness.add(name, Field::from(*byte as u64));
        }
        self
    }

    /// Add a vector of field elements
    pub fn add_vec(mut self, name_prefix: &str, values: &[Field]) -> Self {
        for (i, value) in values.iter().enumerate() {
            let name = format!("{}_{}", name_prefix, i);
            self.witness.add(name, *value);
        }
        self
    }

    /// Build the final witness
    pub fn build(self) -> Witness {
        self.witness
    }
}

/// Public inputs to a circuit
#[derive(Debug, Clone)]
pub struct PublicInputs {
    /// Public variable assignments
    pub assignments: Vec<Field>,
}

impl PublicInputs {
    pub fn new() -> Self {
        Self {
            assignments: Vec::new(),
        }
    }

    pub fn add(&mut self, value: Field) {
        self.assignments.push(value);
    }

    pub fn get(&self, index: usize) -> Option<Field> {
        self.assignments.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    pub fn to_vec(&self) -> Vec<Field> {
        self.assignments.clone()
    }

    pub fn from_vec(values: Vec<Field>) -> Self {
        Self {
            assignments: values,
        }
    }
}

/// Combined witness and public inputs
#[derive(Debug, Clone)]
pub struct Assignment {
    pub witness: Witness,
    pub public_inputs: PublicInputs,
}

impl Assignment {
    pub fn new(witness: Witness, public_inputs: PublicInputs) -> Self {
        Self {
            witness,
            public_inputs,
        }
    }

    /// Get total number of assignments
    pub fn len(&self) -> usize {
        self.witness.len() + self.public_inputs.len()
    }

    /// Check if assignment is complete
    pub fn is_complete(&self, expected_private: usize, expected_public: usize) -> bool {
        self.witness.len() >= expected_private &&
        self.public_inputs.len() >= expected_public
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_builder() {
        let witness = WitnessBuilder::new()
            .add_u64("value", 42)
            .add_bool("flag", true)
            .add_field("custom", Field::from(999))
            .build();

        assert_eq!(witness.get("value"), Some(Field::from(42)));
        assert_eq!(witness.get("flag"), Some(Field::from(1)));
        assert_eq!(witness.get("custom"), Some(Field::from(999)));
        assert_eq!(witness.len(), 3);
    }

    #[test]
    fn test_witness_operations() {
        let mut witness = Witness::new();

        witness.add("a", Field::from(1));
        witness.add("b", Field::from(2));

        assert!(witness.contains("a"));
        assert!(witness.contains("b"));
        assert!(!witness.contains("c"));

        assert_eq!(witness.len(), 2);
    }

    #[test]
    fn test_witness_merge() {
        let mut witness1 = Witness::new();
        witness1.add("a", Field::from(1));

        let mut witness2 = Witness::new();
        witness2.add("b", Field::from(2));

        witness1.merge(witness2);

        assert_eq!(witness1.len(), 2);
        assert_eq!(witness1.get("a"), Some(Field::from(1)));
        assert_eq!(witness1.get("b"), Some(Field::from(2)));
    }

    #[test]
    fn test_public_inputs() {
        let mut public = PublicInputs::new();

        public.add(Field::from(10));
        public.add(Field::from(20));
        public.add(Field::from(30));

        assert_eq!(public.len(), 3);
        assert_eq!(public.get(0), Some(Field::from(10)));
        assert_eq!(public.get(1), Some(Field::from(20)));
        assert_eq!(public.get(2), Some(Field::from(30)));
    }

    #[test]
    fn test_assignment() {
        let witness = WitnessBuilder::new()
            .add_u64("secret", 42)
            .build();

        let mut public = PublicInputs::new();
        public.add(Field::from(100));

        let assignment = Assignment::new(witness, public);

        assert_eq!(assignment.len(), 2);
        assert!(assignment.is_complete(1, 1));
        assert!(!assignment.is_complete(2, 1));
    }

    #[test]
    fn test_witness_from_pairs() {
        let witness = Witness::from_pairs(vec![
            ("x", Field::from(1)),
            ("y", Field::from(2)),
            ("z", Field::from(3)),
        ]);

        assert_eq!(witness.len(), 3);
        assert_eq!(witness.get("x"), Some(Field::from(1)));
        assert_eq!(witness.get("y"), Some(Field::from(2)));
        assert_eq!(witness.get("z"), Some(Field::from(3)));
    }
}
