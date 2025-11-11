/**
 * Shade Framework - Core Types (TypeScript)
 */

/**
 * Field element (BigInt)
 */
export type Field = bigint;

/**
 * Variable in a circuit
 */
export class Variable {
    constructor(public readonly id: number) {}

    toString(): string {
        return `v${this.id}`;
    }
}

/**
 * Linear combination of variables
 */
export class LinearCombination {
    public terms: Array<{ coeff: Field; variable: Variable }> = [];
    public constant: Field = 0n;

    static zero(): LinearCombination {
        return new LinearCombination();
    }

    static fromVariable(v: Variable): LinearCombination {
        const lc = new LinearCombination();
        lc.terms.push({ coeff: 1n, variable: v });
        return lc;
    }

    static fromConstant(c: Field): LinearCombination {
        const lc = new LinearCombination();
        lc.constant = c;
        return lc;
    }

    add(other: LinearCombination): void {
        this.terms.push(...other.terms);
        this.constant += other.constant;
    }

    scale(scalar: Field): void {
        this.terms.forEach(term => {
            term.coeff *= scalar;
        });
        this.constant *= scalar;
    }
}

/**
 * R1CS constraint: A * B = C
 */
export interface R1CSConstraint {
    a: LinearCombination;
    b: LinearCombination;
    c: LinearCombination;
}

/**
 * Witness (private inputs)
 */
export class Witness {
    private assignments: Map<string, Field> = new Map();

    set(name: string, value: Field): void {
        this.assignments.set(name, value);
    }

    get(name: string): Field | undefined {
        return this.assignments.get(name);
    }

    has(name: string): boolean {
        return this.assignments.has(name);
    }

    toObject(): Record<string, string> {
        const obj: Record<string, string> = {};
        this.assignments.forEach((value, key) => {
            obj[key] = value.toString();
        });
        return obj;
    }

    static fromObject(obj: Record<string, string | bigint>): Witness {
        const witness = new Witness();
        for (const [key, value] of Object.entries(obj)) {
            witness.set(key, typeof value === 'string' ? BigInt(value) : value);
        }
        return witness;
    }
}

/**
 * Public inputs
 */
export class PublicInputs {
    public values: Field[] = [];

    add(value: Field): void {
        this.values.push(value);
    }

    get(index: number): Field | undefined {
        return this.values[index];
    }

    length(): number {
        return this.values.length;
    }

    toArray(): Field[] {
        return [...this.values];
    }

    static fromArray(values: Field[]): PublicInputs {
        const pi = new PublicInputs();
        pi.values = [...values];
        return pi;
    }
}

/**
 * Zero-knowledge proof
 */
export interface Proof {
    /** Proof type (groth16, plonk, etc.) */
    type: ProofSystem;

    /** Proof data (serialized) */
    data: Uint8Array;

    /** Public inputs */
    publicInputs: Field[];

    /** Verification key hash */
    vkHash?: string;
}

/**
 * Proof system type
 */
export enum ProofSystem {
    Groth16 = 'groth16',
    Plonk = 'plonk',
    Stark = 'stark',
    Plonky2 = 'plonky2',
    Nova = 'nova',
}

/**
 * Proving key
 */
export interface ProvingKey {
    type: ProofSystem;
    data: Uint8Array;
}

/**
 * Verification key
 */
export interface VerificationKey {
    type: ProofSystem;
    data: Uint8Array;
    hash: string;
}

/**
 * Circuit statistics
 */
export interface CircuitStats {
    constraints: number;
    variables: number;
    publicInputs: number;
    privateInputs: number;
}

/**
 * Proving options
 */
export interface ProvingOptions {
    /** Proof system to use */
    backend?: ProofSystem;

    /** Enable optimization */
    optimize?: boolean;

    /** Optimization level (0-3) */
    optimizationLevel?: number;

    /** Enable parallel proving */
    parallel?: boolean;
}

/**
 * Verification result
 */
export interface VerificationResult {
    /** Whether proof is valid */
    valid: boolean;

    /** Verification time in milliseconds */
    time?: number;

    /** Error message if invalid */
    error?: string;
}

/**
 * Circuit compilation result
 */
export interface CompilationResult {
    /** Compiled circuit */
    circuit: any;

    /** Statistics */
    stats: CircuitStats;

    /** Warnings */
    warnings: string[];

    /** Compilation time in milliseconds */
    time: number;
}

/**
 * Error types
 */
export class CircuitError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'CircuitError';
    }
}

export class ConstraintError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'ConstraintError';
    }
}

export class WitnessError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'WitnessError';
    }
}

export class ProofError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'ProofError';
    }
}

/**
 * Field arithmetic modulo
 */
export const FIELD_MODULUS = 21888242871839275222246405745257275088548364400416034343698204186575808495617n;

/**
 * Field arithmetic helpers
 */
export const FieldMath = {
    add(a: Field, b: Field): Field {
        return (a + b) % FIELD_MODULUS;
    },

    sub(a: Field, b: Field): Field {
        return ((a - b) % FIELD_MODULUS + FIELD_MODULUS) % FIELD_MODULUS;
    },

    mul(a: Field, b: Field): Field {
        return (a * b) % FIELD_MODULUS;
    },

    div(a: Field, b: Field): Field {
        return FieldMath.mul(a, FieldMath.inverse(b));
    },

    inverse(a: Field): Field {
        // Extended Euclidean algorithm
        if (a === 0n) {
            throw new Error('Cannot invert zero');
        }

        let [old_r, r] = [a, FIELD_MODULUS];
        let [old_s, s] = [1n, 0n];

        while (r !== 0n) {
            const quotient = old_r / r;
            [old_r, r] = [r, old_r - quotient * r];
            [old_s, s] = [s, old_s - quotient * s];
        }

        if (old_r > 1n) {
            throw new Error('Field element not invertible');
        }

        if (old_s < 0n) {
            old_s = old_s + FIELD_MODULUS;
        }

        return old_s % FIELD_MODULUS;
    },

    pow(base: Field, exp: bigint): Field {
        if (exp === 0n) return 1n;
        if (exp === 1n) return base;

        let result = 1n;
        let b = base;
        let e = exp;

        while (e > 0n) {
            if (e % 2n === 1n) {
                result = FieldMath.mul(result, b);
            }
            b = FieldMath.mul(b, b);
            e = e / 2n;
        }

        return result;
    },

    isZero(a: Field): boolean {
        return a % FIELD_MODULUS === 0n;
    },

    isOne(a: Field): boolean {
        return a % FIELD_MODULUS === 1n;
    },

    random(): Field {
        // Generate random field element
        const bytes = new Uint8Array(32);
        if (typeof window !== 'undefined' && window.crypto) {
            window.crypto.getRandomValues(bytes);
        } else if (typeof require !== 'undefined') {
            const crypto = require('crypto');
            crypto.randomFillSync(bytes);
        }

        let value = 0n;
        for (let i = 0; i < bytes.length; i++) {
            value = (value << 8n) + BigInt(bytes[i]);
        }

        return value % FIELD_MODULUS;
    },
};
