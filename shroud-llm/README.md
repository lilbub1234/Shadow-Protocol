# Shroud LLM

> **Privacy-First Local LLM Platform**
> Secure document chat with zero-knowledge architecture and advanced privacy features

<p align="center">
  <strong>Your documents. Your AI. Your privacy.</strong>
</p>

---

## 🔒 What is Shroud LLM?

Shroud LLM is a privacy-focused, locally-run LLM platform that enables secure document chat and AI interactions without compromising your data. Built on privacy-first principles, Shroud LLM puts you in complete control of your data, models, and infrastructure.

### Why Shroud LLM?

**Privacy by Design**
- 🔐 **BYOK (Bring Your Own Keys)** - Complete encryption key ownership
- ⏰ **Timer-Based Deletion** - Auto-expire sensitive conversations
- 🖍️ **Smart Redaction** - Automatically detect and redact sensitive information
- 🚫 **Zero Telemetry** - No tracking, no analytics, no data collection
- 🏠 **100% Local** - Run everything on your own hardware

**Powerful & Flexible**
- 📚 Multi-document workspaces with intelligent context management
- 🤖 Custom AI agents with tool integration
- 👥 Multi-user support with granular permissions
- 🔌 Extensive LLM provider support (OpenAI, Anthropic, local models)
- 🗄️ Multiple vector database options
- 📊 Multi-modal support (text, images, PDFs, DOCX, and more)

**Developer Friendly**
- 🛠️ Full REST API for custom integrations
- 🐳 Docker-ready with one-command deployment
- 📦 Easy self-hosting on any platform
- 🔧 Extensible plugin architecture

---

## 🚀 Quick Start

### Prerequisites
- Node.js >= 18
- Docker (optional, for containerized deployment)
- 4GB+ RAM recommended

### Installation

#### Option 1: Docker (Recommended)
```bash
git clone https://github.com/lilbub1234/Shadow-Protocol.git
cd Shadow-Protocol/shroud-llm
docker-compose up -d
```

Visit `http://localhost:3001` to access Shroud LLM.

#### Option 2: Local Development
```bash
git clone https://github.com/lilbub1234/Shadow-Protocol.git
cd Shadow-Protocol/shroud-llm

# Install dependencies
yarn setup

# Start all services
yarn dev:all

# Or start services individually:
# yarn dev:server    # Backend API (port 3001)
# yarn dev:frontend  # Web UI (port 3000)
# yarn dev:collector # Document processor (port 8888)
```

---

## 🔐 Privacy Features

### Timer-Based Deletion
Set automatic expiration for conversations and documents. Perfect for sensitive information that shouldn't persist.

```javascript
// Configure auto-deletion for a workspace
workspace.setAutoDelete({ hours: 24 });
```

### BYOK (Bring Your Own Keys)
Full control over encryption keys. Your keys, your data, your control.

```bash
# Generate your own encryption keys
SHROUD_ENCRYPTION_KEY=your-256-bit-key
SHROUD_SIGNING_KEY=your-signing-key
```

### Smart Redaction
Automatically detect and redact PII, credentials, and sensitive data patterns.

```javascript
// Enable smart redaction
workspace.enableRedaction({
  patterns: ['email', 'ssn', 'credit-card', 'api-key'],
  mode: 'auto' // or 'manual' for review before redaction
});
```

---

## 📖 Core Concepts

### Workspaces
Workspaces are isolated containers for documents and conversations. Each workspace maintains its own context, permissions, and settings - perfect for organizing different projects or sensitivity levels.

### Document Processing
Shroud LLM's document processor handles multiple formats:
- PDF, DOCX, TXT, MD
- Images (OCR support)
- Web pages and URLs
- Code files

### Vector Storage
Choose your preferred vector database:
- LanceDB (default, local)
- Pinecone
- Qdrant
- Chroma
- Milvus
- Weaviate

### LLM Providers
Bring your own LLM or use local models:
- **Local**: Ollama, LM Studio, llama.cpp
- **Cloud**: OpenAI, Anthropic, Cohere, Azure OpenAI
- **Open Source**: Any Hugging Face model

---

## 🛠️ Configuration

### Environment Variables

**Server** (`server/.env`):
```bash
SERVER_PORT=3001
JWT_SECRET=your-jwt-secret-min-12-chars
SIG_KEY=your-signature-key-min-32-chars
SIG_SALT=your-salt-min-32-chars

# LLM Provider (example with Ollama for local)
LLM_PROVIDER=ollama
OLLAMA_BASE_PATH=http://localhost:11434
OLLAMA_MODEL_PREF=llama2

# Vector Database (default: LanceDB)
VECTOR_DB=lancedb

# Privacy Features
ENABLE_AUTO_REDACTION=true
DEFAULT_RETENTION_HOURS=720  # 30 days
```

**Frontend** (`frontend/.env`):
```bash
VITE_API_BASE=http://localhost:3001
```

**Collector** (`collector/.env`):
```bash
SERVER_PORT=8888
```

---

## 🏗️ Architecture

```
┌─────────────────┐
│   Frontend      │  React + Vite
│   (Port 3000)   │  Modern, privacy-focused UI
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Server API    │  Node.js + Express
│   (Port 3001)   │  Business logic, auth, LLM orchestration
└────────┬────────┘
         │
         ├──────────► Vector DB (embeddings)
         ├──────────► LLM Provider
         │
         ▼
┌─────────────────┐
│   Collector     │  Document processing
│   (Port 8888)   │  OCR, parsing, chunking
└─────────────────┘
```

---

## 🤝 Contributing

We welcome contributions! Areas of focus:
- Enhanced privacy features
- New LLM integrations
- UI/UX improvements
- Documentation
- Security audits

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## 📄 License

Shroud LLM is licensed under the **MIT License**. See [LICENSE](./LICENSE) for details.

### Attribution

Shroud LLM is based on [AnythingLLM](https://github.com/Mintplex-Labs/anything-llm) by Mintplex Labs Inc. We're grateful for their excellent foundation. See [NOTICE.md](./NOTICE.md) for complete attribution and [LICENSE.upstream](./LICENSE.upstream) for the original license.

---

## 🔗 Resources

- **Documentation**: [Coming Soon]
- **Issues**: [GitHub Issues](https://github.com/lilbub1234/Shadow-Protocol/issues)
- **Shadow Protocol**: [Main Repository](https://github.com/lilbub1234/Shadow-Protocol)

---

## ⚠️ Security

Found a security vulnerability? Please email security@shadowprotocol.dev or report privately via GitHub Security Advisories. Do not open public issues for security concerns.

---

## 🌟 Roadmap

- [ ] End-to-end encryption for all data at rest
- [ ] Federated learning capabilities
- [ ] Advanced audit logging
- [ ] Hardware security module (HSM) integration
- [ ] Mobile applications (iOS/Android)
- [ ] Browser extension
- [ ] Blockchain-based verification

---

<p align="center">
  <strong>Built with privacy in mind. Always.</strong>
</p>
