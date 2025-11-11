"""
Shade Framework - Field Arithmetic (Python)

Finite field arithmetic modulo BN254 prime.
"""

# BN254 scalar field modulus
FIELD_MODULUS = 21888242871839275222246405745257275088548364400416034343698204186575808495617


class Field:
    """Field element in BN254 scalar field."""

    def __init__(self, value: int):
        """Create a field element."""
        self.value = value % FIELD_MODULUS

    def __add__(self, other: 'Field') -> 'Field':
        """Add two field elements."""
        return Field((self.value + other.value) % FIELD_MODULUS)

    def __sub__(self, other: 'Field') -> 'Field':
        """Subtract two field elements."""
        return Field((self.value - other.value + FIELD_MODULUS) % FIELD_MODULUS)

    def __mul__(self, other: 'Field') -> 'Field':
        """Multiply two field elements."""
        return Field((self.value * other.value) % FIELD_MODULUS)

    def __truediv__(self, other: 'Field') -> 'Field':
        """Divide two field elements."""
        return self * other.inverse()

    def __pow__(self, exponent: int) -> 'Field':
        """Raise field element to a power."""
        if exponent == 0:
            return Field(1)
        if exponent < 0:
            return self.inverse() ** (-exponent)

        result = Field(1)
        base = Field(self.value)
        exp = exponent

        while exp > 0:
            if exp % 2 == 1:
                result = result * base
            base = base * base
            exp //= 2

        return result

    def __neg__(self) -> 'Field':
        """Negate field element."""
        return Field((FIELD_MODULUS - self.value) % FIELD_MODULUS)

    def __eq__(self, other: object) -> bool:
        """Check equality."""
        if not isinstance(other, Field):
            return False
        return self.value == other.value

    def __repr__(self) -> str:
        """String representation."""
        return f"Field({self.value})"

    def __int__(self) -> int:
        """Convert to integer."""
        return self.value

    def inverse(self) -> 'Field':
        """Compute multiplicative inverse using extended Euclidean algorithm."""
        if self.value == 0:
            raise ValueError("Cannot invert zero")

        def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
            if a == 0:
                return b, 0, 1
            gcd, x1, y1 = extended_gcd(b % a, a)
            x = y1 - (b // a) * x1
            y = x1
            return gcd, x, y

        _, x, _ = extended_gcd(self.value, FIELD_MODULUS)
        return Field(x % FIELD_MODULUS)

    def is_zero(self) -> bool:
        """Check if field element is zero."""
        return self.value == 0

    def is_one(self) -> bool:
        """Check if field element is one."""
        return self.value == 1

    @staticmethod
    def zero() -> 'Field':
        """Return zero element."""
        return Field(0)

    @staticmethod
    def one() -> 'Field':
        """Return one element."""
        return Field(1)

    @staticmethod
    def random() -> 'Field':
        """Generate a random field element."""
        import secrets
        return Field(secrets.randbelow(FIELD_MODULUS))


class FieldMath:
    """Field arithmetic operations."""

    @staticmethod
    def add(a: Field, b: Field) -> Field:
        """Add two field elements."""
        return a + b

    @staticmethod
    def sub(a: Field, b: Field) -> Field:
        """Subtract two field elements."""
        return a - b

    @staticmethod
    def mul(a: Field, b: Field) -> Field:
        """Multiply two field elements."""
        return a * b

    @staticmethod
    def div(a: Field, b: Field) -> Field:
        """Divide two field elements."""
        return a / b

    @staticmethod
    def pow(base: Field, exp: int) -> Field:
        """Raise to power."""
        return base ** exp

    @staticmethod
    def inverse(a: Field) -> Field:
        """Compute multiplicative inverse."""
        return a.inverse()

    @staticmethod
    def neg(a: Field) -> Field:
        """Negate."""
        return -a

    @staticmethod
    def is_zero(a: Field) -> bool:
        """Check if zero."""
        return a.is_zero()

    @staticmethod
    def is_one(a: Field) -> bool:
        """Check if one."""
        return a.is_one()

    @staticmethod
    def random() -> Field:
        """Generate random field element."""
        return Field.random()


# Convenience function for creating field elements
def F(value: int) -> Field:
    """Create a field element."""
    return Field(value)
