# Contributing to Shroud Framework

Thank you for your interest in contributing to Shroud Framework! We're building the future of privacy infrastructure together, and your contributions make that possible.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. All contributors must adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md). Be respectful, collaborative, and constructive.

## How to Contribute

There are many ways to contribute to Shroud Framework:

### 1. Report Bugs
- Search [existing issues](https://github.com/shroud-protocol/shroud/issues) to avoid duplicates
- Use the bug report template
- Include reproduction steps, expected behavior, and actual behavior
- Provide system information (OS, Shroud version, etc.)

### 2. Suggest Features
- Open a [feature request](https://github.com/shroud-protocol/shroud/issues/new?template=feature_request.md)
- Describe the problem you're trying to solve
- Explain your proposed solution
- Consider alternatives and trade-offs

### 3. Improve Documentation
- Fix typos, clarify explanations, add examples
- Write tutorials and guides
- Improve API documentation
- Translate documentation to other languages

### 4. Write Code
- Fix bugs, implement features, optimize performance
- Add tests and benchmarks
- Improve error messages
- Refactor and clean up code

### 5. Build Plugins
- Create custom proof system backends
- Build circuit gadgets
- Develop blockchain adapters
- Design UI themes

### 6. Create Templates
- Build privacy application templates
- Share example circuits
- Contribute to the template library

## Development Setup

### Prerequisites
- **Rust**: 1.70+ ([install](https://rustup.rs/))
- **Node.js**: 18+ ([install](https://nodejs.org/))
- **Python**: 3.9+ ([install](https://python.org/))
- **PostgreSQL**: 14+ ([install](https://postgresql.org/))
- **Git**: Latest version

### Clone the Repository
```bash
git clone https://github.com/shroud-protocol/shroud.git
cd shroud
```

### Install Dependencies
```bash
# Rust dependencies
cargo build

# Visual builder frontend
cd visual-builder/frontend
npm install

# Visual builder backend
cd ../backend
cargo build

# Python SDK
cd ../../sdk/python
pip install -e .
```

### Run Tests
```bash
# All tests
cargo test --all

# Specific crate
cargo test -p shroud-core

# With coverage
cargo tarpaulin --out Html --output-dir coverage
```

### Run Visual Builder Locally
```bash
# Start backend
cd visual-builder/backend
cargo run

# In another terminal, start frontend
cd visual-builder/frontend
npm run dev
```

### Run CLI
```bash
cargo run --bin shroud -- --help
```

## Contribution Workflow

### 1. Fork the Repository
Click the "Fork" button on GitHub to create your copy.

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name

# Or for bug fixes:
git checkout -b fix/bug-description
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test additions/changes
- `perf/` - Performance improvements

### 3. Make Your Changes
- Write clean, readable code
- Follow our coding standards (see below)
- Add tests for new functionality
- Update documentation as needed
- Keep commits atomic and focused

### 4. Write Good Commit Messages
```
type(scope): Short description (50 chars max)

Longer explanation if needed (wrap at 72 characters).
Explain what changed and why, not how.

Fixes #123
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance

Examples:
```
feat(visual-builder): Add AI-powered circuit suggestions

The AI assistant now analyzes user's circuit design and suggests
optimizations in real-time. This reduces circuit size by 20-40%
on average.

Closes #456

---

fix(core): Fix integer overflow in constraint optimizer

The optimizer could overflow when processing large circuits.
Added checked arithmetic and proper error handling.

Fixes #789
```

### 5. Run Quality Checks
```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --all -- -D warnings

# Tests
cargo test --all

# Benchmarks (if relevant)
cargo bench

# Check docs
cargo doc --no-deps --open
```

### 6. Push Your Branch
```bash
git push origin feature/your-feature-name
```

### 7. Create a Pull Request
- Go to your fork on GitHub
- Click "Pull Request"
- Select your branch
- Fill out the PR template
- Link related issues
- Request review from maintainers

### 8. Address Review Feedback
- Respond to comments
- Make requested changes
- Push additional commits
- Request re-review when ready

### 9. Celebrate!
Once your PR is merged, you're officially a Shroud Framework contributor! 🎉

## Coding Standards

### Rust Code
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Pass `cargo clippy` without warnings
- Write comprehensive documentation
- Add examples to documentation
- Use descriptive variable names
- Prefer explicit over implicit
- Handle errors properly (no `.unwrap()` in library code)

```rust
// Good
pub fn prove_circuit(circuit: &Circuit, witness: Witness) -> Result<Proof, ShadeError> {
    let compiled = compile_with_optimization(circuit)?;
    let proof = generate_proof(&compiled, witness)?;
    Ok(proof)
}

// Bad
pub fn prove_circuit(circuit: &Circuit, witness: Witness) -> Proof {
    let compiled = compile_with_optimization(circuit).unwrap();
    generate_proof(&compiled, witness).unwrap()
}
```

### TypeScript/JavaScript Code
- Use TypeScript for type safety
- Follow [Airbnb Style Guide](https://github.com/airbnb/javascript)
- Use `prettier` for formatting
- Use `eslint` for linting
- Write JSDoc comments

```typescript
/**
 * Generate a zero-knowledge proof for the given circuit
 *
 * @param circuit - The circuit to prove
 * @param witness - Private inputs (witness)
 * @returns A promise resolving to the generated proof
 * @throws {CircuitError} If circuit is invalid
 * @throws {WitnessError} If witness doesn't satisfy constraints
 *
 * @example
 * const proof = await proveCircuit(myCircuit, myWitness);
 * const valid = await verifyProof(proof);
 */
async function proveCircuit(
  circuit: Circuit,
  witness: Witness
): Promise<Proof> {
  // Implementation
}
```

### Python Code
- Follow [PEP 8](https://peps.python.org/pep-0008/)
- Use type hints
- Use `black` for formatting
- Use `pylint` for linting
- Write docstrings

```python
def prove_circuit(circuit: Circuit, witness: Witness) -> Proof:
    """
    Generate a zero-knowledge proof for the given circuit.

    Args:
        circuit: The circuit to prove
        witness: Private inputs (witness)

    Returns:
        The generated proof

    Raises:
        CircuitError: If circuit is invalid
        WitnessError: If witness doesn't satisfy constraints

    Example:
        >>> proof = prove_circuit(my_circuit, my_witness)
        >>> valid = verify_proof(proof)
        >>> assert valid
    """
    # Implementation
```

## Testing Guidelines

### Unit Tests
- Test individual functions in isolation
- Mock dependencies
- Cover edge cases
- Test error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_hash_deterministic() {
        let input = vec![Field::from(42)];
        let hash1 = poseidon_hash(&input);
        let hash2 = poseidon_hash(&input);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_poseidon_hash_different_inputs() {
        let input1 = vec![Field::from(1)];
        let input2 = vec![Field::from(2)];
        let hash1 = poseidon_hash(&input1);
        let hash2 = poseidon_hash(&input2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_poseidon_hash_empty_input() {
        let input = vec![];
        let result = poseidon_hash(&input);
        // Should not panic, returns deterministic value
        assert!(result != Field::zero());
    }
}
```

### Integration Tests
- Test interactions between components
- Use realistic scenarios
- Test happy paths and error paths

```rust
#[tokio::test]
async fn test_end_to_end_proof_generation() {
    // Setup
    let engine = ProvingEngine::new();
    let circuit = create_test_circuit();
    let witness = generate_valid_witness();

    // Compile
    let compiled = engine.compile_circuit(circuit).await.unwrap();

    // Prove
    let requirements = ProofRequirements::default();
    let proof = engine.prove(&compiled, witness, requirements).await.unwrap();

    // Verify
    let public_inputs = extract_public_inputs(&proof);
    let valid = engine.verify(&proof, &public_inputs).await.unwrap();

    assert!(valid);
}
```

### Property-Based Tests
- Test invariants and properties
- Generate random inputs
- Find edge cases automatically

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_merkle_proof_soundness(leaf in any::<u64>(), path_len in 1..20usize) {
        let tree = generate_merkle_tree(path_len);
        let path = tree.prove(leaf);
        let root = tree.root();

        // Property: Valid proofs always verify
        assert!(verify_merkle_proof(leaf, &path, root));
    }

    #[test]
    fn test_merkle_proof_completeness(
        leaf in any::<u64>(),
        fake_root in any::<u64>(),
        path_len in 1..20usize
    ) {
        let tree = generate_merkle_tree(path_len);
        let path = tree.prove(leaf);

        // Property: Proofs with wrong root don't verify
        if fake_root != tree.root() {
            assert!(!verify_merkle_proof(leaf, &path, fake_root));
        }
    }
}
```

## Performance Considerations

### Optimization Guidelines
- Profile before optimizing
- Focus on hot paths
- Use benchmarks to measure improvements
- Document performance characteristics

```rust
// Add benchmarks
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_poseidon_hash(c: &mut Criterion) {
    let input = vec![Field::from(42)];

    c.bench_function("poseidon_hash", |b| {
        b.iter(|| poseidon_hash(black_box(&input)))
    });
}

criterion_group!(benches, bench_poseidon_hash);
criterion_main!(benches);
```

### Performance Budget
- Circuit compilation: < 5s for 1M constraints
- Proof generation: < 5s for 1M constraints
- Proof verification: < 10ms
- Visual builder response: < 100ms

## Security Guidelines

### Cryptographic Code
- Use audited libraries
- Never implement your own crypto
- Use constant-time operations
- Avoid timing side-channels
- Use secure random number generation
- Clear sensitive data from memory

```rust
// Good: Constant-time comparison
use subtle::ConstantTimeEq;

fn verify_signature(sig: &Signature, expected: &Signature) -> bool {
    sig.ct_eq(expected).into()
}

// Bad: Variable-time comparison (timing side-channel)
fn verify_signature(sig: &Signature, expected: &Signature) -> bool {
    sig == expected
}
```

### Input Validation
- Validate all inputs
- Check ranges and bounds
- Reject malformed data
- Fail securely

### Error Handling
- Don't leak sensitive information in errors
- Log security-relevant events
- Rate-limit operations

## Documentation Guidelines

### Code Documentation
- Document all public APIs
- Include examples
- Explain non-obvious behavior
- Document panics and errors
- Link to related functions

### Guides and Tutorials
- Start with motivation
- Explain concepts clearly
- Include complete examples
- Provide next steps

### README Updates
- Keep README.md current
- Update feature lists
- Add new examples
- Maintain accuracy

## Release Process

We follow [Semantic Versioning](https://semver.org/):
- **Major version** (1.0.0): Breaking changes
- **Minor version** (0.1.0): New features (backwards compatible)
- **Patch version** (0.0.1): Bug fixes

## Community

### Get Help
- **Discord**: [discord.gg/shroud](https://discord.gg/shroud)
- **Forum**: [forum.shadeframework.io](https://forum.shadeframework.io)
- **GitHub Discussions**: Ask questions

### Stay Updated
- **Blog**: [blog.shadeframework.io](https://blog.shadeframework.io)
- **Twitter**: [@ShadeFramework](https://twitter.com/ShadeFramework)
- **Newsletter**: Monthly updates

### Recognition
- All contributors are listed in [CONTRIBUTORS.md](./CONTRIBUTORS.md)
- Significant contributions highlighted in release notes
- Annual contributor awards

## License

By contributing to Shroud Framework, you agree that your contributions will be licensed under the [MIT License](./LICENSE).

---

**Thank you for contributing to Shroud Framework!**

Together, we're building infrastructure that makes privacy accessible to everyone. Your contributions matter.
