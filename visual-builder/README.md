# Shroud Visual Builder

The Visual Builder is Shroud Framework's revolutionary no-code interface for designing zero-knowledge circuits and privacy applications.

## Overview

Traditional ZK development requires deep cryptographic knowledge and weeks of learning. The Visual Builder democratizes privacy by enabling anyone to create sophisticated ZK applications through an intuitive drag-and-drop interface.

## Key Features

### 1. Node-Based Circuit Editor
- **Drag & Drop**: Intuitive canvas interface
- **Live Validation**: Real-time constraint checking
- **Visual Debugging**: See data flow through your circuit
- **Auto-Layout**: Automatically organize complex circuits

### 2. Component Library
Pre-built, audited components for common operations:

**Cryptographic Primitives**
- Poseidon Hash
- SHA-256, Blake2b
- ECDSA Signature Verification
- EdDSA Signature Verification

**Constraints**
- Range Check (a ≤ x ≤ b)
- Equality (x = y)
- Comparison (x < y, x > y)
- Membership (x ∈ set)

**Data Structures**
- Merkle Tree Proof
- Accumulator Operations
- Nullifier Generation
- Commitment Schemes

**Control Flow**
- Conditional (if-then-else)
- Switch (multi-branch)
- Loops (fixed iteration)
- Assertions

### 3. Template System
Start from battle-tested privacy app templates:

- **Private Mixer**: Confidential asset transfers
- **Anonymous Voting**: Secure elections with verifiable tallies
- **Credential System**: Privacy-preserving identity
- **Encrypted Messaging**: E2E encrypted communications
- **DeFi Privacy**: Private trading and lending
- **Supply Chain**: Confidential business logic

### 4. Live Preview
- **Instant Testing**: Test circuits without leaving the UI
- **Sample Data**: Auto-generate test inputs
- **Proof Visualization**: Watch proof generation step-by-step
- **Performance Metrics**: See constraint count, proof time, verification time

### 5. Code Export
Generate production-ready code in multiple languages:
- **Rust**: Native performance
- **TypeScript**: Web and Node.js applications
- **Python**: Scripting and data science
- **WASM**: Browser-native execution

Export includes:
- Optimized circuit implementation
- Complete test suite
- Deployment scripts
- Documentation

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Frontend (React)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │
│  │ Canvas       │  │ Component    │  │ Properties       │ │
│  │ Editor       │  │ Library      │  │ Panel            │ │
│  └──────────────┘  └──────────────┘  └──────────────────┘ │
└────────────────────────────┬────────────────────────────────┘
                             │ WebSocket (real-time)
┌────────────────────────────┴────────────────────────────────┐
│                    Backend (Rust + TypeScript)              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │
│  │ Circuit      │  │ Constraint   │  │ Code             │ │
│  │ Generator    │  │ Validator    │  │ Generator        │ │
│  └──────────────┘  └──────────────┘  └──────────────────┘ │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────┴────────────────────────────────┐
│                  Shroud Core Engine                          │
│  (Circuit Compilation, Proof Generation, Verification)      │
└─────────────────────────────────────────────────────────────┘
```

## User Interface

### Main Canvas
```
┌────────────────────────────────────────────────────────────────┐
│ File  Edit  View  Circuit  Tools  Help          [▶ Test] [📤]  │
├───────────┬────────────────────────────────────┬───────────────┤
│           │                                    │               │
│ Components│         Circuit Canvas             │  Properties   │
│           │                                    │               │
│ ○ Inputs  │    ┌─────────┐                    │  Node: Hash   │
│ ○ Outputs │    │ Secret  │                    │  Type: Poseidon│
│ ⚡ Crypto  │    │  Input  │────────┐           │               │
│ ✓ Constraints│  └─────────┘        │           │  Inputs: 1    │
│ 📊 Data   │                        ▼           │  Outputs: 1   │
│ 🔀 Control│              ┌──────────────┐      │               │
│ 📦 Advanced│             │   Poseidon   │      │  ☑ Optimize   │
│           │              │     Hash     │      │  ☐ Debug      │
│ Templates │              └──────┬───────┘      │               │
│ ○ Mixer   │                     │              │  [Apply]      │
│ ○ Voting  │                     ▼              │               │
│ ○ Creds   │              ┌──────────────┐      │               │
│           │              │   Public     │      │  Stats:       │
│           │              │   Output     │      │  Constraints: │
│           │              └──────────────┘      │       1,247   │
│           │                                    │               │
├───────────┴────────────────────────────────────┴───────────────┤
│ Circuit: private_note.shroud │ 1,247 constraints │ Modified     │
└────────────────────────────────────────────────────────────────┘
```

### Component Properties
When a node is selected, the properties panel shows:
- **Type**: Component type (Hash, Range Check, etc.)
- **Configuration**: Type-specific settings
- **Inputs**: Connected input nodes
- **Outputs**: Connected output nodes
- **Optimization**: Enable/disable optimizations
- **Documentation**: Built-in help text

### Test Panel
```
┌─────────────────────────────────────────────────────────┐
│                    Test Circuit                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Inputs:                                                │
│  ┌─────────────────────────────────────────────────┐   │
│  │ note_content: "Hello, this is my private note!" │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  [Generate Sample Data]  [Load from File]              │
│                                                         │
│  [▶ Generate Proof]                                    │
│                                                         │
│  Results:                                               │
│  ✅ Constraints satisfied                              │
│  ✅ Proof generated (235ms)                            │
│  ✅ Proof verified (3ms)                               │
│                                                         │
│  Public Outputs:                                        │
│  ┌─────────────────────────────────────────────────┐   │
│  │ note_hash: 0x4a5f2c...8d3e (Field element)      │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  [View Proof]  [Export Verifier]  [Share Result]       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Component System

### Component Definition
Each visual component maps to a circuit gadget:

```typescript
interface Component {
    id: string;
    type: ComponentType;
    position: { x: number; y: number };
    config: ComponentConfig;
    inputs: Port[];
    outputs: Port[];
}

interface Port {
    id: string;
    label: string;
    dataType: DataType;
    connectedTo?: string; // Connected port ID
}

enum DataType {
    Field,         // Field element
    Boolean,       // Single bit
    Array,         // Array of field elements
    Struct,        // Custom structure
}
```

### Creating Custom Components
Users can package circuits as reusable components:

1. Design circuit in canvas
2. Select nodes to package
3. Click "Create Component"
4. Define inputs/outputs
5. Save to library

The component becomes available in the library for future use.

## Circuit Generation

### Visual → Circuit Translation

When user creates this visual circuit:
```
[Secret Input] → [Hash] → [Public Output]
      ↓
[Range Check]
```

The Visual Builder generates:
```rust
#[shroud::circuit]
pub struct GeneratedCircuit {
    #[private]
    secret_input: Field,

    #[public]
    public_output: Field,
}

impl Circuit for GeneratedCircuit {
    fn constraints(&self) -> Result<()> {
        // Range check constraint
        self.assert_range(self.secret_input, MIN, MAX)?;

        // Hash constraint
        let hash = poseidon_hash(&[self.secret_input]);

        // Output constraint
        self.assert_equal(hash, self.public_output)?;

        Ok(())
    }
}
```

### Optimization
The Visual Builder automatically applies optimizations:
- **Dead Node Elimination**: Remove unconnected nodes
- **Constraint Reduction**: Merge compatible constraints
- **Common Subgraph**: Reuse identical sub-circuits
- **Type Inference**: Minimize type conversions

## Live Validation

### Real-Time Error Detection
The Visual Builder validates circuits as you build:

**Type Errors**
- "Cannot connect Field output to Boolean input"
- "Component expects 2 inputs, but only 1 is connected"

**Logic Errors**
- "Public output is never constrained"
- "Cyclic dependency detected"
- "Constraint may be under-constrained"

**Performance Warnings**
- "Circuit exceeds 1M constraints (slow proving)"
- "Consider using a range proof gadget instead"
- "This hash function is not ZK-friendly"

### Constraint Preview
Hover over any node to see:
- Number of constraints it generates
- Estimated contribution to total proof time
- Optimization opportunities

## Templates

### Private Mixer Template
```
        ┌──────────────┐
        │  Deposit     │
        │  - Amount    │
        │  - Secret    │
        └──────┬───────┘
               │
        ┌──────▼───────┐
        │  Commitment  │
        │  (Poseidon)  │
        └──────┬───────┘
               │
        ┌──────▼───────┐
        │  Merkle Tree │
        │  Accumulator │
        └──────┬───────┘
               │
        ┌──────▼───────┐
        │  Withdrawal  │
        │  - Nullifier │
        │  - Recipient │
        └──────────────┘
```

Generates complete mixer with:
- Deposit contract
- Merkle tree accumulator
- Withdrawal circuit with nullifier
- Relayer support
- Frontend UI

### Anonymous Voting Template
```
    ┌─────────────┐
    │ Voter       │
    │ Eligibility │──────┐
    └─────────────┘      │
                         ▼
    ┌─────────────┐  ┌──────────┐
    │ Vote        │─▶│ Nullifier│
    │ Commitment  │  │ Check    │
    └─────────────┘  └────┬─────┘
                          │
                    ┌─────▼──────┐
                    │  Tallying  │
                    │  (Private) │
                    └────────────┘
```

Generates voting system with:
- Eligibility proof
- Ballot commitment
- Double-vote prevention
- Private tallying
- Public result verification

## Export Formats

### Rust Export
```rust
// Generated by Shroud Visual Builder
// Project: private_note
// Generated: 2025-01-15 14:32:00 UTC

use shroud::prelude::*;

/// Private note circuit
///
/// Proves knowledge of a note's content without revealing it
#[shroud::circuit]
pub struct PrivateNote {
    /// The note content (private)
    #[private]
    note_content: String,

    /// Hash of the note (public)
    #[public]
    note_hash: Field,
}

impl Circuit for PrivateNote {
    fn constraints(&self) -> Result<()> {
        // Validate note length (10-1000 characters)
        let length = self.note_content.len();
        self.assert_range(length, 10, 1000)?;

        // Compute hash
        let hash = poseidon_hash(&self.note_content);

        // Constrain public output
        self.assert_equal(hash, self.note_hash)?;

        Ok(())
    }
}

// Generated test suite
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_note() {
        let note = "This is my private note";
        let hash = poseidon_hash(note);

        let circuit = PrivateNote {
            note_content: note.to_string(),
            note_hash: hash,
        };

        let proof = circuit.prove().unwrap();
        assert!(proof.verify());
    }
}
```

### TypeScript Export
```typescript
// Generated by Shroud Visual Builder

import { Circuit, Field, poseidonHash } from '@shroud/sdk';

/**
 * Private note circuit
 *
 * Proves knowledge of a note's content without revealing it
 */
export class PrivateNote extends Circuit {
    constructor(
        private noteContent: string,
        public noteHash: Field
    ) {
        super();
    }

    constraints(): void {
        // Validate note length
        const length = this.noteContent.length;
        this.assertRange(length, 10, 1000);

        // Compute hash
        const hash = poseidonHash(this.noteContent);

        // Constrain public output
        this.assertEqual(hash, this.noteHash);
    }
}

// Example usage
async function main() {
    const note = "This is my private note";
    const hash = poseidonHash(note);

    const circuit = new PrivateNote(note, hash);
    const proof = await circuit.prove();
    const valid = await proof.verify();

    console.log(`Proof valid: ${valid}`);
}
```

## Keyboard Shortcuts

- **Cmd/Ctrl + N**: New circuit
- **Cmd/Ctrl + O**: Open circuit
- **Cmd/Ctrl + S**: Save circuit
- **Cmd/Ctrl + Z**: Undo
- **Cmd/Ctrl + Y**: Redo
- **Cmd/Ctrl + C**: Copy selection
- **Cmd/Ctrl + V**: Paste
- **Cmd/Ctrl + A**: Select all
- **Delete/Backspace**: Delete selection
- **Space + Drag**: Pan canvas
- **Cmd/Ctrl + Scroll**: Zoom
- **Cmd/Ctrl + T**: Test circuit
- **Cmd/Ctrl + E**: Export code

## Technical Implementation

### Frontend Stack
- **React 18**: UI framework
- **React Flow**: Node-based editor
- **TailwindCSS**: Styling
- **Monaco Editor**: Code editor
- **Vite**: Build tooling

### Backend Stack
- **Rust**: Core circuit generation
- **Actix-web**: HTTP server
- **WebSocket**: Real-time updates
- **PostgreSQL**: Project storage

### Communication Protocol
```typescript
// WebSocket messages
type Message =
    | { type: 'validate_circuit'; circuit: VisualCircuit }
    | { type: 'generate_code'; circuit: VisualCircuit; language: Language }
    | { type: 'test_circuit'; circuit: VisualCircuit; inputs: Inputs }
    | { type: 'optimize_circuit'; circuit: VisualCircuit };

type Response =
    | { type: 'validation_result'; valid: boolean; errors: Error[] }
    | { type: 'generated_code'; code: string; language: Language }
    | { type: 'test_result'; success: boolean; proof?: Proof; metrics: Metrics }
    | { type: 'optimized_circuit'; circuit: VisualCircuit; stats: OptimizationStats };
```

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Opera 76+

## Local Development

```bash
# Start backend
cd visual-builder/backend
cargo run

# Start frontend
cd visual-builder/frontend
npm install
npm run dev

# Open browser
open http://localhost:3000
```

## Future Enhancements

### Phase 2
- **Collaborative Editing**: Real-time multi-user editing
- **Version Control**: Git-like versioning for circuits
- **Component Marketplace**: Buy/sell custom components
- **Mobile App**: iOS and Android apps

### Phase 3
- **AI Suggestions**: AI recommends optimizations in real-time
- **Natural Language**: Describe circuits in plain English
- **Formal Verification**: Prove circuit properties
- **Visual Debugging**: Step through proof generation visually

---

The Visual Builder transforms zero-knowledge development from weeks of learning to minutes of building. It's privacy infrastructure for everyone.
