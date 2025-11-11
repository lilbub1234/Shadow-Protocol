"""
Shroud Framework - Circuit Module (Python)

Provides Python bindings for building zero-knowledge circuits.
"""

from typing import List, Optional, Callable
from dataclasses import dataclass
from abc import ABC, abstractmethod


@dataclass
class Variable:
    """Represents a variable in a circuit."""
    id: int

    def __repr__(self) -> str:
        return f"v{self.id}"


class LinearCombination:
    """Linear combination of variables with coefficients."""

    def __init__(self):
        self.terms: List[tuple[int, Variable]] = []
        self.constant: int = 0

    @staticmethod
    def zero() -> 'LinearCombination':
        """Create a zero linear combination."""
        return LinearCombination()

    @staticmethod
    def from_variable(var: Variable) -> 'LinearCombination':
        """Create a linear combination from a single variable."""
        lc = LinearCombination()
        lc.terms.append((1, var))
        return lc

    @staticmethod
    def from_constant(c: int) -> 'LinearCombination':
        """Create a linear combination from a constant."""
        lc = LinearCombination()
        lc.constant = c
        return lc

    def add(self, other: 'LinearCombination') -> None:
        """Add another linear combination."""
        self.terms.extend(other.terms)
        self.constant += other.constant

    def scale(self, scalar: int) -> None:
        """Scale by a constant."""
        self.terms = [(coeff * scalar, var) for coeff, var in self.terms]
        self.constant *= scalar


@dataclass
class R1CSConstraint:
    """R1CS constraint: A * B = C"""
    a: LinearCombination
    b: LinearCombination
    c: LinearCombination


class ConstraintSystem:
    """Constraint system for building circuits."""

    def __init__(self):
        self.variables: List[Variable] = []
        self.assignments: dict[int, int] = {}
        self.constraints: List[R1CSConstraint] = []
        self.num_public = 0
        self.num_private = 0
        self._next_var = 0

    def alloc_variable(self, value: Optional[int] = None) -> Variable:
        """Allocate a new variable."""
        var = Variable(self._next_var)
        self._next_var += 1
        self.variables.append(var)

        if value is not None:
            self.assignments[var.id] = value

        return var

    def alloc_public_input(self, value: int) -> Variable:
        """Allocate a public input variable."""
        self.num_public += 1
        return self.alloc_variable(value)

    def alloc_private_input(self, value: int) -> Variable:
        """Allocate a private input variable (witness)."""
        self.num_private += 1
        return self.alloc_variable(value)

    def enforce_constraint(
        self,
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
    ) -> None:
        """Add a constraint: A * B = C"""
        self.constraints.append(R1CSConstraint(a, b, c))

    def enforce_equal(self, a: LinearCombination, b: LinearCombination) -> None:
        """Enforce that two linear combinations are equal."""
        # a = b  =>  (a - b) * 1 = 0
        diff = LinearCombination()
        diff.terms.extend(a.terms)
        diff.constant = a.constant

        neg_b = LinearCombination()
        neg_b.terms = [(-coeff, var) for coeff, var in b.terms]
        neg_b.constant = -b.constant

        diff.add(neg_b)

        one = LinearCombination.from_constant(1)
        zero = LinearCombination.zero()

        self.enforce_constraint(diff, one, zero)

    def enforce_mul(self, a: Variable, b: Variable, c: Variable) -> None:
        """Enforce multiplication: a * b = c"""
        self.enforce_constraint(
            LinearCombination.from_variable(a),
            LinearCombination.from_variable(b),
            LinearCombination.from_variable(c),
        )

    def get_value(self, var: Variable) -> Optional[int]:
        """Get variable assignment."""
        return self.assignments.get(var.id)

    def set_value(self, var: Variable, value: int) -> None:
        """Set variable assignment."""
        self.assignments[var.id] = value

    def is_satisfied(self) -> bool:
        """Check if all constraints are satisfied."""
        FIELD_MODULUS = 21888242871839275222246405745257275088548364400416034343698204186575808495617

        for constraint in self.constraints:
            # Evaluate A
            a_val = constraint.a.constant
            for coeff, var in constraint.a.terms:
                if var.id not in self.assignments:
                    return False
                a_val += coeff * self.assignments[var.id]
            a_val %= FIELD_MODULUS

            # Evaluate B
            b_val = constraint.b.constant
            for coeff, var in constraint.b.terms:
                if var.id not in self.assignments:
                    return False
                b_val += coeff * self.assignments[var.id]
            b_val %= FIELD_MODULUS

            # Evaluate C
            c_val = constraint.c.constant
            for coeff, var in constraint.c.terms:
                if var.id not in self.assignments:
                    return False
                c_val += coeff * self.assignments[var.id]
            c_val %= FIELD_MODULUS

            # Check A * B = C
            if (a_val * b_val) % FIELD_MODULUS != c_val:
                return False

        return True

    def num_constraints(self) -> int:
        """Get number of constraints."""
        return len(self.constraints)

    def stats(self) -> dict:
        """Get circuit statistics."""
        return {
            "constraints": len(self.constraints),
            "variables": len(self.variables),
            "public_inputs": self.num_public,
            "private_inputs": self.num_private,
        }


class Circuit(ABC):
    """Base class for zero-knowledge circuits."""

    def __init__(self):
        self.cs = ConstraintSystem()

    @abstractmethod
    def build_constraints(self) -> None:
        """Define circuit constraints. Override this method."""
        pass

    def prove(self, witness: 'Witness') -> 'Proof':
        """Generate a proof for this circuit."""
        # Build constraints if not already built
        if self.cs.num_constraints() == 0:
            self.build_constraints()

        # Assign witness values
        self._assign_witness(witness)

        # Check constraints are satisfied
        if not self.cs.is_satisfied():
            raise ValueError("Constraints not satisfied with given witness")

        # Generate proof (would call into Rust backend)
        from .proof import Proof
        return Proof(self.cs)

    def verify(self, proof: 'Proof', public_inputs: 'PublicInputs') -> bool:
        """Verify a proof."""
        return proof.verify(public_inputs)

    def stats(self) -> dict:
        """Get circuit statistics."""
        return self.cs.stats()

    def _assign_witness(self, witness: 'Witness') -> None:
        """Assign witness values to circuit."""
        for name, value in witness.assignments.items():
            # Would map witness names to variables
            pass


class SquareCircuit(Circuit):
    """Example circuit: prove x^2 = y"""

    def __init__(self, x: int, y: int):
        super().__init__()
        self.x = x
        self.y = y

    def build_constraints(self) -> None:
        # Allocate variables
        x_var = self.cs.alloc_private_input(self.x)
        y_var = self.cs.alloc_public_input(self.y)

        # Enforce x * x = y
        self.cs.enforce_mul(x_var, x_var, y_var)


class HashPreimageCircuit(Circuit):
    """Example circuit: prove knowledge of hash preimage"""

    def __init__(self, preimage: int, hash_value: int):
        super().__init__()
        self.preimage = preimage
        self.hash_value = hash_value

    def build_constraints(self) -> None:
        from .gadgets import PoseidonHash

        # Allocate variables
        preimage_var = self.cs.alloc_private_input(self.preimage)
        hash_var = self.cs.alloc_public_input(self.hash_value)

        # Compute hash
        hash_gadget = PoseidonHash(self.cs, [preimage_var])
        computed_hash = hash_gadget.output()

        # Enforce hash equals expected value
        self.cs.enforce_equal(
            LinearCombination.from_variable(computed_hash),
            LinearCombination.from_variable(hash_var),
        )


def create_circuit() -> ConstraintSystem:
    """Create a new constraint system."""
    return ConstraintSystem()


# Export Witness and Proof for type hints
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from .witness import Witness, PublicInputs
    from .proof import Proof
