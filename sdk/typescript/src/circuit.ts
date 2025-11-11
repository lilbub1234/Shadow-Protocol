/**
 * Shade Framework - Circuit Builder (TypeScript)
 */

import { Field, Variable, Witness, PublicInputs, Proof } from './types';
import { ConstraintSystem } from './constraint-system';

/**
 * Base circuit class
 * Extend this to define your own circuits
 */
export abstract class Circuit {
    protected cs: ConstraintSystem;

    constructor() {
        this.cs = new ConstraintSystem();
    }

    /**
     * Define circuit constraints
     * Override this method to build your circuit
     */
    abstract buildConstraints(): void;

    /**
     * Generate a proof for this circuit
     */
    async prove(witness: Witness): Promise<Proof> {
        // Build constraints if not already built
        if (this.cs.constraints.length === 0) {
            this.buildConstraints();
        }

        // Assign witness values
        this.cs.assignWitness(witness);

        // Check constraints are satisfied
        if (!this.cs.isSatisfied()) {
            throw new Error('Constraints not satisfied with given witness');
        }

        // Generate proof using backend
        return this.cs.generateProof();
    }

    /**
     * Verify a proof
     */
    async verify(proof: Proof, publicInputs: PublicInputs): Promise<boolean> {
        return this.cs.verifyProof(proof, publicInputs);
    }

    /**
     * Get circuit statistics
     */
    getStats() {
        return {
            constraints: this.cs.constraints.length,
            variables: this.cs.variables.length,
            publicInputs: this.cs.numPublic,
            privateInputs: this.cs.numPrivate,
        };
    }

    /**
     * Export verifier contract
     */
    exportVerifier(format: 'solidity' | 'rust' | 'typescript' = 'solidity'): string {
        return this.cs.exportVerifier(format);
    }

    // Helper methods for building constraints

    /**
     * Allocate a new variable
     */
    protected allocVariable(value?: Field): Variable {
        return this.cs.allocVariable(value);
    }

    /**
     * Allocate a public input
     */
    protected allocPublicInput(value: Field): Variable {
        return this.cs.allocPublicInput(value);
    }

    /**
     * Allocate a private input (witness)
     */
    protected allocPrivateInput(value: Field): Variable {
        return this.cs.allocPrivateInput(value);
    }

    /**
     * Enforce multiplication: a * b = c
     */
    protected enforceMul(a: Variable, b: Variable, c: Variable): void {
        this.cs.enforceMul(a, b, c);
    }

    /**
     * Enforce equality: a = b
     */
    protected enforceEqual(a: Variable, b: Variable): void {
        this.cs.enforceEqual(a, b);
    }

    /**
     * Assert a constraint is satisfied
     */
    protected assert(condition: boolean, message?: string): void {
        if (!condition) {
            throw new Error(message || 'Assertion failed');
        }
    }
}

/**
 * Simple example circuit
 */
export class SquareCircuit extends Circuit {
    private input: Variable;
    private output: Variable;

    constructor(inputValue: Field, outputValue: Field) {
        super();
        this.input = this.allocPrivateInput(inputValue);
        this.output = this.allocPublicInput(outputValue);
    }

    buildConstraints(): void {
        // Enforce: input * input = output
        this.enforceMul(this.input, this.input, this.output);
    }
}

/**
 * Hash preimage circuit
 */
export class HashPreimageCircuit extends Circuit {
    private preimage: Variable;
    private hash: Variable;

    constructor(preimageValue: Field, hashValue: Field) {
        super();
        this.preimage = this.allocPrivateInput(preimageValue);
        this.hash = this.allocPublicInput(hashValue);
    }

    buildConstraints(): void {
        // Import hash gadget
        const { poseidonHash } = require('./gadgets');

        // Compute hash of preimage
        const computedHash = poseidonHash(this.cs, [this.preimage]);

        // Enforce: computed hash equals expected hash
        this.enforceEqual(computedHash, this.hash);
    }
}

/**
 * Range proof circuit
 */
export class RangeProofCircuit extends Circuit {
    private value: Variable;
    private min: number;
    private max: number;

    constructor(value: Field, min: number, max: number) {
        super();
        this.value = this.allocPrivateInput(value);
        this.min = min;
        this.max = max;
    }

    buildConstraints(): void {
        const { rangeCheck } = require('./gadgets');

        // Enforce: min <= value <= max
        rangeCheck(this.cs, this.value, this.min, this.max);
    }
}

/**
 * Conditional circuit
 */
export class ConditionalCircuit extends Circuit {
    private condition: Variable;
    private trueValue: Variable;
    private falseValue: Variable;
    private output: Variable;

    constructor(
        condition: Field,
        trueValue: Field,
        falseValue: Field,
        expectedOutput: Field
    ) {
        super();
        this.condition = this.allocPrivateInput(condition);
        this.trueValue = this.allocPrivateInput(trueValue);
        this.falseValue = this.allocPrivateInput(falseValue);
        this.output = this.allocPublicInput(expectedOutput);
    }

    buildConstraints(): void {
        const { conditionalSelect } = require('./gadgets');

        // Output = condition ? trueValue : falseValue
        const selected = conditionalSelect(
            this.cs,
            this.condition,
            this.trueValue,
            this.falseValue
        );

        this.enforceEqual(selected, this.output);
    }
}

/**
 * Circuit builder with fluent API
 */
export class CircuitBuilder {
    private circuit: Circuit;

    constructor() {
        // Create anonymous circuit class
        this.circuit = new (class extends Circuit {
            buildConstraints(): void {
                // Constraints added via builder
            }
        })();
    }

    /**
     * Add a constraint
     */
    constraint(fn: (cs: ConstraintSystem) => void): this {
        fn(this.circuit['cs']);
        return this;
    }

    /**
     * Build the circuit
     */
    build(): Circuit {
        this.circuit.buildConstraints();
        return this.circuit;
    }
}

// Export helper function
export function createCircuit(): CircuitBuilder {
    return new CircuitBuilder();
}
