"""
Shroud Framework - Python SDK
Copyright (c) 2025 Shroud Protocol Contributors
Licensed under MIT License

Zero-knowledge proof framework for privacy-preserving applications.
"""

__version__ = "0.1.0"
__author__ = "Shroud Protocol Contributors"
__license__ = "MIT"

from .circuit import Circuit, ConstraintSystem, Variable
from .gadgets import PoseidonHash, RangeCheck, MerkleProof
from .witness import Witness, PublicInputs
from .proof import Proof, ProofSystem
from .field import Field, FieldMath

__all__ = [
    # Core
    "Circuit",
    "ConstraintSystem",
    "Variable",

    # Gadgets
    "PoseidonHash",
    "RangeCheck",
    "MerkleProof",

    # Witness
    "Witness",
    "PublicInputs",

    # Proof
    "Proof",
    "ProofSystem",

    # Field
    "Field",
    "FieldMath",
]
