# Getting Started with Shade Framework

Welcome to Shade Framework! This guide will take you from zero to your first privacy-preserving application in under 10 minutes.

## Installation

### Prerequisites

Shade Framework works on all major platforms:
- **Linux**: Ubuntu 20.04+, Arch, Fedora
- **macOS**: 11.0+ (Big Sur and later)
- **Windows**: Windows 10+ (via WSL2 recommended)

### Quick Install

Choose your preferred installation method:

#### Option 1: Universal Installer (Recommended)
```bash
curl -fsSL https://get.shadeframework.io | sh
```

This will:
- Detect your platform
- Install Shade CLI
- Set up PATH configuration
- Download core libraries
- Verify installation

#### Option 2: Package Managers

**Rust (Cargo)**
```bash
cargo install shade-cli
```

**Node.js (npm)**
```bash
npm install -g @shade/cli
```

**Python (pip)**
```bash
pip install shade-framework
```

### Verify Installation

```bash
shade --version
# Output: Shade Framework v0.1.0
```

## Your First Privacy App: Private Note

Let's build a simple private note application where users can prove they know the content of a note without revealing it.

### Method 1: Visual Builder (No Code)

1. **Launch the Visual Builder**
```bash
shade build --visual
```

This opens the Shade Visual Builder in your browser at `http://localhost:3000`.

2. **Create a New Project**
   - Click "New Project"
   - Name: "Private Note"
   - Template: "Start from Scratch"
   - Click "Create"

3. **Design Your Circuit**

Drag components from the left panel onto the canvas:

```
[Secret Input] → [Hash (Poseidon)] → [Public Output]
     ↓
 [Length Check]
 (Note: 10-1000 chars)
```

- Drag **"Secret Input"** component (labeled "note_content")
- Drag **"Hash"** component (choose "Poseidon" from dropdown)
- Connect: `note_content` → `hash.input`
- Drag **"Range Check"** component
- Connect: `note_content.length` → `range_check.value`
- Set range_check min=10, max=1000
- Drag **"Public Output"** (labeled "note_hash")
- Connect: `hash.output` → `note_hash`

4. **Test Your Circuit**
   - Click "Test" in top-right
   - Enter sample note: "Hello, this is my private note!"
   - Click "Generate Proof"
   - See: ✅ Proof generated successfully (235ms)
   - Verification: ✅ Verified (3ms)

5. **Export Code**
   - Click "Export"
   - Choose language: Rust / TypeScript / Python
   - Choose deployment: Ethereum / Solana / Standalone
   - Click "Download"

You now have production-ready code!

### Method 2: AI Assistant

1. **Describe What You Want**
```bash
shade generate
```

When prompted, enter:
```
Create a private note application where users can prove they
wrote a note without revealing its content. The note should be
10-1000 characters and use a secure hash function.
```

2. **Review Generated Design**

The AI will present:
```
Analyzing requirements...
✅ Detected needs:
   - Secret input validation (10-1000 chars)
   - Cryptographic commitment (hash)
   - Public verification

Suggested circuit:
   - Input: note_content (private)
   - Constraint: 10 ≤ length ≤ 1000
   - Hash: Poseidon (ZK-friendly)
   - Output: note_hash (public)

Circuit complexity: ~1,200 constraints
Proof time: ~150ms
Verification: ~3ms

Proceed with generation? (y/n)
```

3. **Generate and Test**
```bash
# Type 'y' to proceed
# AI generates complete implementation

# Test the circuit
shade test

# Deploy if desired
shade deploy --chain ethereum
```

### Method 3: Code (For Developers)

1. **Initialize Project**
```bash
shade init private-note --template basic
cd private-note
```

2. **Edit Circuit** (`src/circuit.rs`)
```rust
use shade::prelude::*;

#[shade::circuit]
pub struct PrivateNote {
    // Private input
    #[private]
    note_content: String,

    // Public output
    #[public]
    note_hash: Field,
}

impl Circuit for PrivateNote {
    fn constraints(&self) -> Result<()> {
        // Validate note length
        let length = self.note_content.len();
        self.assert_range(length, 10, 1000)?;

        // Hash the note
        let hash = poseidon_hash(&self.note_content);

        // Constrain public output
        self.assert_equal(hash, self.note_hash)?;

        Ok(())
    }
}
```

3. **Write Tests** (`tests/circuit_test.rs`)
```rust
use shade::testing::*;

#[test]
fn test_valid_note() {
    let note = "This is my private note";
    let hash = poseidon_hash(note);

    let circuit = PrivateNote {
        note_content: note.to_string(),
        note_hash: hash,
    };

    // Generate proof
    let proof = circuit.prove().unwrap();

    // Verify proof
    assert!(proof.verify());
}

#[test]
fn test_note_too_short() {
    let note = "short";
    let hash = poseidon_hash(note);

    let circuit = PrivateNote {
        note_content: note.to_string(),
        note_hash: hash,
    };

    // Should fail constraint
    assert!(circuit.prove().is_err());
}
```

4. **Run Tests**
```bash
shade test
```

Output:
```
Running tests...
✅ test_valid_note ... passed (180ms)
✅ test_note_too_short ... passed (5ms)

2 tests passed
Circuit complexity: 1,247 constraints
```

5. **Build for Production**
```bash
shade build --release --optimize
```

Output:
```
Optimizing circuit...
  - Dead constraint elimination: -45 constraints
  - Constant folding: -23 constraints
  - Gadget fusion: -12 constraints

Final circuit: 1,167 constraints (6.4% reduction)
Proof time: 142ms
Verification time: 2.8ms

Build complete: ./build/private_note
```

## Next Steps: Building More Complex Apps

### Private Voting System

```bash
shade create voting --type anonymous --eligibility token-holder
```

This generates a complete voting application with:
- Voter registration
- Anonymous ballot casting
- Public tally verification
- Admin controls

### Token Mixer

```bash
shade create mixer --asset ETH --anonymity-set 1000
```

This creates a privacy mixer with:
- Deposit contracts
- Merkle tree accumulator
- Withdrawal proofs
- Relayer network support

### Credential System

```bash
shade create credentials --schema age-verification
```

This builds a credential issuance and verification system:
- Issuer signing
- Selective disclosure
- Revocation support
- Privacy-preserving verification

## Understanding Shade Concepts

### Circuits
A **circuit** is a mathematical representation of a computation. It consists of:
- **Variables**: Inputs, outputs, and intermediate values
- **Constraints**: Equations that must be satisfied
- **Witnesses**: Private data that satisfies constraints

### Proofs
A **proof** demonstrates that you know a witness satisfying circuit constraints without revealing the witness.

**Example**:
```
Circuit: I know a number x such that SHA256(x) = known_hash
Proof: Convinces verifier this is true without revealing x
```

### Verification
**Verification** checks a proof's validity. It's:
- Fast (milliseconds)
- Compact (single function call)
- Public (anyone can verify)

### Recursive Proofs
**Recursive proofs** allow proofs to verify other proofs, enabling:
- Proof aggregation (many proofs → one proof)
- Incremental computation (update proof as computation progresses)
- Scalability (constant verification time regardless of computation)

## Configuration

### Project Structure

A Shade project looks like:
```
private-note/
├── shade.toml           # Project configuration
├── src/
│   ├── circuit.rs       # Main circuit definition
│   └── lib.rs           # Library exports
├── tests/
│   └── circuit_test.rs  # Test suite
├── deploy/
│   └── ethereum.sol     # Deployment contracts (generated)
└── build/               # Build artifacts
```

### shade.toml

```toml
[project]
name = "private-note"
version = "0.1.0"
author = "Your Name"

[circuit]
# Choose proving system
backend = "groth16"  # Options: groth16, plonk, stark, plonky2, nova

# Optimization level
optimize = true
optimization_level = 2  # 0-3

[build]
# Output formats
targets = ["rust", "typescript", "wasm"]

# Deployment targets
deploy_chains = ["ethereum", "polygon"]

[security]
# Require range checks on all inputs
strict_inputs = true

# Maximum circuit size (prevent DoS)
max_constraints = 1_000_000

[performance]
# Enable parallel proving
parallel_proving = true
num_threads = 8

# Cache compiled circuits
cache_enabled = true
```

## CLI Commands Reference

### Project Management
```bash
shade init <name>           # Create new project
shade build                 # Build project
shade build --visual        # Open visual builder
shade clean                 # Clean build artifacts
```

### Development
```bash
shade test                  # Run tests
shade test --verbose        # Detailed output
shade bench                 # Run benchmarks
shade optimize              # Analyze and suggest optimizations
```

### Proof Operations
```bash
shade prove                 # Generate proof
shade verify <proof>        # Verify proof
shade export-verifier       # Export verifier code
```

### Deployment
```bash
shade deploy                # Deploy to configured chains
shade deploy --chain <name> # Deploy to specific chain
shade deploy --testnet      # Deploy to testnets
```

### AI Assistant
```bash
shade generate              # AI-assisted generation
shade suggest               # Get optimization suggestions
shade explain <circuit>     # Explain circuit design
```

### Utilities
```bash
shade info                  # Show project info
shade stats                 # Circuit statistics
shade visualize             # Visualize circuit graph
shade upgrade               # Update Shade Framework
```

## Getting Help

### Documentation
- **Full Docs**: [docs.shadeframework.io](https://docs.shadeframework.io)
- **API Reference**: [api.shadeframework.io](https://api.shadeframework.io)
- **Examples**: Browse the [examples](../../examples) directory

### Community
- **Discord**: [discord.gg/shade](https://discord.gg/shade)
- **Forum**: [forum.shadeframework.io](https://forum.shadeframework.io)
- **GitHub Discussions**: Ask questions and share projects

### Built-in Help
```bash
shade help                  # List all commands
shade help <command>        # Detailed command help
shade doctor                # Diagnose installation issues
```

## Troubleshooting

### Build Failures

**Error**: `Failed to compile circuit`
```bash
# Check circuit syntax
shade check

# Verbose build output
shade build --verbose

# Clean and rebuild
shade clean && shade build
```

### Proof Generation Fails

**Error**: `Constraint not satisfied`
```bash
# Debug mode shows which constraint failed
shade prove --debug

# Visualize constraint dependency graph
shade visualize --constraints
```

### Performance Issues

**Slow proof generation**
```bash
# Enable optimizations
shade build --optimize --release

# Use parallel proving
shade prove --parallel

# Profile hot spots
shade profile
```

## What's Next?

Now that you've built your first privacy app, explore:

1. **[Visual Builder Tutorial](./visual-builder.md)** - Master the no-code interface
2. **[Circuit Design Patterns](./circuit-patterns.md)** - Learn best practices
3. **[zkVM Programming](./zkvm-programming.md)** - Build with universal computation
4. **[Privacy App Templates](./templates.md)** - Explore pre-built applications
5. **[Plugin Development](./plugin-development.md)** - Extend Shade Framework

Welcome to the Shade community. Let's build a more private future together.
