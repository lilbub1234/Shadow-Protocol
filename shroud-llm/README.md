<p align="center">
  <img src="https://i.postimg.cc/N2bhJDxS/image.png" alt="Shroud LLM Logo" width="200"/>
</p>

<h1 align="center">Shroud LLM</h1>

<p align="center">
  <strong>Your Privacy-First, Locally-Run LLM Platform</strong>
</p>

<p align="center">
  <em>Take back control of your data. Chat with documents privately. No cloud. No tracking. Just you and your AI.</em>
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-why-shroud-llm">Why Shroud</a> •
  <a href="#-features">Features</a> •
  <a href="#-privacy-first">Privacy</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-documentation">Docs</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/privacy-first-green?style=for-the-badge" alt="Privacy First"/>
  <img src="https://img.shields.io/badge/100%25-local-blue?style=for-the-badge" alt="100% Local"/>
  <img src="https://img.shields.io/badge/license-MIT-orange?style=for-the-badge" alt="MIT License"/>
  <img src="https://img.shields.io/badge/node-%3E%3D18-brightgreen?style=for-the-badge" alt="Node 18+"/>
</p>

---

## 🔒 What is Shroud LLM?

**Shroud LLM** is a privacy-focused, fully local AI platform that lets you chat with your documents using powerful language models—without ever sending your data to the cloud.

Think of it as **ChatGPT for your private documents**, but everything stays on **your computer**, under **your control**.

### The Problem

- ☁️ Cloud AI services see all your data
- 📊 Your conversations are tracked and analyzed
- 🔓 Sensitive documents leave your control
- 💰 Monthly subscriptions add up
- 🌐 Requires internet connectivity

### The Shroud Solution

- 🏠 **100% Local** - Everything runs on your machine
- 🔒 **Zero Knowledge** - Your data never leaves your computer
- 🚫 **No Telemetry** - No tracking, no analytics, no spying
- 💰 **Free Forever** - No subscriptions, no hidden costs
- ⚡ **Offline Ready** - Works without internet (with local models)

---

## 🌟 Why Shroud LLM?

<table>
<tr>
<td width="33%" align="center">

### 🛡️ Privacy First

Your documents, conversations, and data **never leave your machine**. No cloud uploads, no external servers, no data collection.

</td>
<td width="33%" align="center">

### 🏠 Fully Local

Run powerful AI models like **Llama**, **Mistral**, or **GPT-4-All** right on your computer. No internet required.

</td>
<td width="33%" align="center">

### 🔓 Open Source

Completely open source and auditable. See exactly what the code does. No hidden tracking or backdoors.

</td>
</tr>
<tr>
<td width="33%" align="center">

### ⏰ Auto-Delete

Set conversations to **automatically delete** after a time period. Perfect for sensitive information.

</td>
<td width="33%" align="center">

### 🔐 BYOK

**Bring Your Own Keys**. Use your own encryption keys for zero-knowledge security.

</td>
<td width="33%" align="center">

### 🖍️ Smart Redaction

Automatically detect and **redact PII** like emails, SSNs, credit cards, and API keys.

</td>
</tr>
</table>

---

## ✨ Features

### 📚 Document Intelligence

- **Multi-Format Support**: PDF, DOCX, TXT, Markdown, images (with OCR), and more
- **Smart Chunking**: Intelligently splits large documents for better context
- **Vector Search**: Lightning-fast semantic search across all your documents
- **Citations**: See exactly where AI got information from your documents

### 🤖 Flexible AI Models

Choose your preferred AI model:

- **Local Models**: Ollama, LM Studio, llama.cpp (run offline!)
- **Cloud Models**: OpenAI, Anthropic, Cohere, Azure OpenAI
- **Open Source**: Any Hugging Face model
- **Mix & Match**: Use different models for different workspaces

### 🗂️ Workspace Organization

- **Multiple Workspaces**: Organize documents by project, topic, or sensitivity
- **Per-Workspace Settings**: Each workspace can use different AI models
- **Document Folders**: Organize files just like your desktop
- **Thread Management**: Keep conversations organized by topic

### 🔐 Privacy Features (Unique to Shroud!)

#### ⏰ Timer-Based Deletion
Set conversations and documents to **auto-delete** after:
- 24 hours (for sensitive chats)
- 7 days (for temporary projects)
- 30 days (default retention)
- Custom periods

#### 🔑 BYOK (Bring Your Own Keys)
- Provide your own **256-bit encryption keys**
- **Zero-knowledge** architecture
- Keys stored **only in memory** (never on disk)
- Automatic key rotation support

#### 🖍️ Smart Redaction
Automatically detects and redacts:
- 📧 Email addresses
- 🔢 Social Security Numbers
- 💳 Credit card numbers
- 📞 Phone numbers
- 🔑 API keys and passwords
- 🏦 Bitcoin/Ethereum addresses
- ...and more!

### 👥 Multi-User Support

- **Role-Based Access**: Admin, Manager, User roles
- **Workspace Permissions**: Control who sees what
- **User Management**: Easy admin dashboard
- **SSO Support**: Single sign-on integration

### 🎨 Modern Interface

- **Clean Design**: Intuitive, clutter-free UI
- **Dark Mode**: Easy on the eyes
- **Drag & Drop**: Simple document uploads
- **Keyboard Shortcuts**: Power user friendly
- **Mobile Responsive**: Works on tablets and phones

---

## 🚀 Quick Start

### Option 1: Docker (Recommended - 2 Minutes)

```bash
# Clone the repository
git clone https://github.com/lilbub1234/Shadow-Protocol.git
cd Shadow-Protocol/shroud-llm

# Start with Docker
docker-compose up -d

# Open in browser
open http://localhost:3001
```

**That's it!** Shroud LLM is now running locally on your machine.

### Option 2: Local Development (5 Minutes)

```bash
# Prerequisites: Node.js 18+ and Yarn

# Clone the repository
git clone https://github.com/lilbub1234/Shadow-Protocol.git
cd Shadow-Protocol/shroud-llm

# Install dependencies
yarn setup

# Start all services
yarn dev:all

# Open in browser
open http://localhost:3000
```

### Option 3: One-Click Desktop App (Coming Soon)

Download the desktop app for **Mac**, **Windows**, or **Linux**:
- 📥 Download from releases
- 📦 Double-click to install
- 🚀 Start chatting privately

---

## 📖 How It Works

1. **📄 Upload Documents**: Drop PDFs, Word docs, or text files into a workspace
2. **🔍 Smart Processing**: Shroud extracts and indexes content locally
3. **💬 Ask Questions**: Chat naturally about your documents
4. **🤖 AI Responds**: Get accurate answers with citations
5. **🔒 Stay Private**: All data stays on your machine—forever

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Your Computer                        │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Frontend   │  │    Server    │  │  Collector   │  │
│  │   (React)    │◄─┤  (Node.js)   │◄─┤  (Document   │  │
│  │              │  │              │  │  Processor)  │  │
│  └──────────────┘  └──────┬───────┘  └──────────────┘  │
│                            │                             │
│                    ┌───────▼────────┐                    │
│                    │  Vector Store  │                    │
│                    │   (LanceDB)    │                    │
│                    └────────────────┘                    │
│                            │                             │
│                    ┌───────▼────────┐                    │
│                    │   Local LLM    │                    │
│                    │  (Ollama/etc)  │                    │
│                    └────────────────┘                    │
│                                                           │
│              🔒 100% Local • Zero Cloud ☁️               │
└─────────────────────────────────────────────────────────┘
```

---

## 🎯 Use Cases

### For Individuals

- 📚 **Research**: Chat with research papers, books, and articles
- 💼 **Work**: Analyze contracts, reports, and business documents
- 📝 **Writing**: Reference notes and outlines while writing
- 🎓 **Learning**: Study textbooks and course materials
- 🔐 **Privacy**: Handle sensitive personal documents

### For Professionals

- ⚖️ **Legal**: Review contracts and legal documents privately
- 🏥 **Healthcare**: Analyze medical records (HIPAA compliant)
- 💰 **Finance**: Process financial documents securely
- 🔬 **Research**: Work with proprietary research data
- 🏢 **Enterprise**: Handle confidential business information

### For Teams

- 👥 **Collaboration**: Share workspaces with team members
- 🔒 **Compliance**: Meet GDPR, CCPA, HIPAA requirements
- 📊 **Knowledge Base**: Build private company knowledge bases
- 🎯 **Project Management**: Organize project documentation
- 🛡️ **Security**: Keep sensitive data in-house

---

## 🔐 Privacy First

### What We DO

- ✅ Run everything **locally on your computer**
- ✅ Store data **only in your local database**
- ✅ Process documents **entirely offline** (with local models)
- ✅ Give you **complete control** over your data
- ✅ Allow you to **delete everything** anytime
- ✅ Provide **encryption options** for sensitive data
- ✅ **Open source** all code for transparency

### What We DON'T Do

- ❌ Send data to external servers (unless you choose cloud AI)
- ❌ Track your usage or analytics
- ❌ Collect telemetry or crash reports (opt-in only)
- ❌ Share data with third parties
- ❌ Require an internet connection (with local models)
- ❌ Sell your data or show ads
- ❌ Lock you into our platform

### Privacy Features

| Feature | Description | Status |
|---------|-------------|--------|
| **Local-First** | All data stored locally | ✅ Default |
| **Zero Telemetry** | No tracking by default | ✅ Default |
| **Timer Deletion** | Auto-delete after X hours | ✅ Available |
| **BYOK Encryption** | Your own encryption keys | ✅ Available |
| **Smart Redaction** | Auto-redact PII | ✅ Available |
| **Offline Mode** | Works without internet | ✅ With local models |
| **Open Source** | Fully auditable code | ✅ MIT License |

---

## 📦 Installation

### System Requirements

**Minimum**:
- **OS**: Windows 10, macOS 11, Ubuntu 20.04, or newer
- **RAM**: 4 GB (8 GB recommended)
- **Storage**: 2 GB free space (more for documents)
- **CPU**: Any modern processor
- **Node.js**: Version 18 or higher

**For Local AI Models**:
- **RAM**: 8 GB minimum (16 GB+ recommended)
- **CPU**: Modern multi-core processor
- **GPU**: Optional but recommended (NVIDIA, AMD, or Apple Silicon)

### Step-by-Step Installation

#### 1️⃣ Install Dependencies

**Node.js 18+**:
```bash
# macOS (using Homebrew)
brew install node

# Ubuntu/Debian
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Windows (using Chocolatey)
choco install nodejs
```

**Yarn Package Manager**:
```bash
npm install -g yarn
```

#### 2️⃣ Clone Repository

```bash
git clone https://github.com/lilbub1234/Shadow-Protocol.git
cd Shadow-Protocol/shroud-llm
```

#### 3️⃣ Install & Configure

```bash
# Install all dependencies
yarn setup

# This will:
# - Install server dependencies
# - Install frontend dependencies
# - Install document collector dependencies
# - Copy example .env files
# - Set up database
```

#### 4️⃣ Configure AI Model

Edit `server/.env.development`:

**Option A: Use Local Models (Private & Free)**
```bash
# For Ollama (recommended)
LLM_PROVIDER='ollama'
OLLAMA_BASE_PATH='http://localhost:11434'
OLLAMA_MODEL_PREF='llama2'

# Install Ollama: https://ollama.ai
# Then: ollama pull llama2
```

**Option B: Use Cloud Models (Requires API Key)**
```bash
# For OpenAI
LLM_PROVIDER='openai'
OPEN_AI_KEY='your-api-key-here'
OPEN_MODEL_PREF='gpt-4'
```

#### 5️⃣ Enable Privacy Features (Optional)

Add to `server/.env.development`:

```bash
# Timer-Based Deletion
ENABLE_TIMER_DELETION=true
DEFAULT_RETENTION_HOURS=720  # 30 days

# BYOK Encryption
ENABLE_BYOK=true

# Smart Redaction
ENABLE_AUTO_REDACTION=true
```

#### 6️⃣ Start Shroud LLM

```bash
# Start all services
yarn dev:all

# Or start individually:
yarn dev:server     # Backend (port 3001)
yarn dev:frontend   # Frontend (port 3000)
yarn dev:collector  # Document processor (port 8888)
```

#### 7️⃣ Open in Browser

```
http://localhost:3000
```

**Create your first workspace and start chatting privately! 🎉**

---

## 🐳 Docker Deployment

### Quick Deploy

```bash
cd shroud-llm
docker-compose up -d
```

### Custom Configuration

```bash
# Copy environment file
cp docker/.env.example docker/.env

# Edit configuration
nano docker/.env

# Start services
docker-compose up -d
```

### Docker Commands

```bash
# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Restart services
docker-compose restart

# Update to latest version
git pull
docker-compose up -d --build
```

---

## 📚 Documentation

### User Guides

- [Getting Started](docs/getting-started.md)
- [Document Upload](docs/uploading-documents.md)
- [Workspace Management](docs/workspaces.md)
- [AI Model Configuration](docs/ai-models.md)
- [Privacy Features](server/privacy/README.md)

### Advanced Topics

- [Privacy Features Guide](server/privacy/README.md)
- [API Documentation](docs/api.md)
- [Self-Hosting Guide](BARE_METAL.md)
- [Docker Deployment](docker/HOW_TO_USE_DOCKER.md)
- [Troubleshooting](docs/troubleshooting.md)

### Developer Resources

- [Contributing Guide](CONTRIBUTING.md)
- [Development Setup](docs/development.md)
- [Architecture Overview](docs/architecture.md)
- [API Reference](docs/api-reference.md)

---

## 🔧 Configuration

### AI Model Options

Shroud LLM supports many AI providers:

**Local (Privacy-Focused)**:
- 🦙 Ollama (recommended for privacy)
- 💻 LM Studio
- 🔗 llama.cpp
- 🤖 LocalAI

**Cloud (Requires API Keys)**:
- 🤖 OpenAI (GPT-4, GPT-3.5)
- 🧠 Anthropic (Claude)
- ☁️ Azure OpenAI
- 🌟 Cohere
- ⚡ Groq
- 🔥 Fireworks AI
- ...and 20+ more!

### Vector Database Options

Choose your vector storage:
- **LanceDB** (default, local)
- Pinecone
- Qdrant
- Chroma
- Milvus
- Weaviate

### Environment Variables

Key configuration options in `server/.env`:

```bash
# Server
SERVER_PORT=3001

# Security
JWT_SECRET="your-secret-key"
SIG_KEY="your-signing-key"
SIG_SALT="your-salt"

# LLM Provider
LLM_PROVIDER='ollama'
OLLAMA_MODEL_PREF='llama2'

# Vector Database
VECTOR_DB='lancedb'

# Privacy Features
ENABLE_TIMER_DELETION=true
ENABLE_BYOK=true
ENABLE_AUTO_REDACTION=true
```

---

## 🛡️ Security

### Secure by Design

- 🔒 **Encrypted Storage**: Sensitive data encrypted at rest
- 🔑 **JWT Authentication**: Secure user sessions
- 🛡️ **CSRF Protection**: Protection against attacks
- 🔐 **Bcrypt Hashing**: Secure password storage
- ⚠️ **Input Validation**: All inputs sanitized
- 🚫 **No Secrets in Code**: Environment-based config

### Security Best Practices

1. **Strong Passwords**: Use secure JWT secrets
2. **HTTPS Only**: Use SSL/TLS in production
3. **Firewall**: Restrict access to necessary ports
4. **Updates**: Keep dependencies up to date
5. **Backups**: Regular database backups
6. **Monitoring**: Watch logs for suspicious activity

### Report Security Issues

Found a vulnerability? Please report privately:

- 🔒 [GitHub Security Advisories](https://github.com/lilbub1234/Shadow-Protocol/security/advisories)
- ✉️ Email: security@shadowprotocol.dev

**Do not** open public issues for security concerns.

---

## 🤝 Contributing

We welcome contributions! Shroud LLM is built by the community, for the community.

### Areas We Need Help

- 🔐 **Privacy Features**: BYOK, redaction, encryption
- 🌍 **Translations**: Help us reach more users
- 📝 **Documentation**: Improve guides and tutorials
- 🐛 **Bug Reports**: Find and report issues
- ✨ **Features**: Suggest and build new features
- 🧪 **Testing**: Help us test and validate

### How to Contribute

1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes
4. **Test** thoroughly
5. **Submit** a pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## 📝 License

**Shroud LLM** is licensed under the **MIT License**.

- ✅ Use commercially
- ✅ Modify as needed
- ✅ Distribute freely
- ✅ Private use
- ❗ Must include license and copyright notice

See [LICENSE](LICENSE) for full terms.

### Attribution

Shroud LLM is based on [AnythingLLM](https://github.com/Mintplex-Labs/anything-llm) by Mintplex Labs Inc. We're grateful for their excellent foundation.

See [NOTICE.md](NOTICE.md) for complete attribution.

---

## 🌟 Roadmap

### Current Version (1.0)
- ✅ Local LLM support
- ✅ Document chat
- ✅ Multi-workspace
- ✅ Privacy features
- ✅ Docker deployment

### Coming Soon (1.1)
- [ ] Desktop apps (Mac, Windows, Linux)
- [ ] Mobile apps (iOS, Android)
- [ ] Advanced encryption
- [ ] Federated workspaces
- [ ] Improved redaction

### Future (2.0+)
- [ ] Browser extension
- [ ] Voice input/output
- [ ] Collaborative editing
- [ ] Plugin system
- [ ] Blockchain verification

---

## 💬 Community & Support

### Get Help

- 📖 [Documentation](docs/)
- 💬 [GitHub Discussions](https://github.com/lilbub1234/Shadow-Protocol/discussions)
- 🐛 [Issue Tracker](https://github.com/lilbub1234/Shadow-Protocol/issues)
- 📧 Email: support@shadowprotocol.dev

### Stay Updated

- ⭐ Star this repository
- 👀 Watch for updates
- 🐦 Follow on Twitter: [@ShroudLLM](https://twitter.com/ShroudLLM)
- 📧 Newsletter: [Subscribe](https://shroudllm.dev/newsletter)

---

## ❓ FAQ

### Is Shroud LLM really free?

**Yes!** Shroud LLM is 100% free and open source. No subscriptions, no hidden fees. If you use local AI models (like Ollama), you don't even need API keys.

### Does my data leave my computer?

**Only if you choose cloud AI models** (like OpenAI or Anthropic). If you use local models (like Ollama or LM Studio), all data stays on your computer. We never send data to our servers because we don't have any!

### Can I use it offline?

**Yes!** With local AI models (Ollama, LM Studio), Shroud LLM works completely offline. Perfect for sensitive work or areas with limited internet.

### What about API keys for cloud providers?

If you choose to use cloud AI (like OpenAI), you'll need your own API key. These keys are stored locally in your `.env` file and never sent anywhere except directly to the AI provider you choose.

### How is this different from ChatGPT?

ChatGPT sends all your data to OpenAI's servers. Shroud LLM runs **on your computer** and works with **your documents**. Plus, you can use completely local AI models for true privacy.

### What AI models can I use?

**Local**: Llama 2, Llama 3, Mistral, Phi, GPT4All, and any Ollama model
**Cloud**: GPT-4, Claude, Gemini, and 20+ other providers

### Is it hard to set up?

**Not at all!** With Docker, it's literally 2 commands:
```bash
docker-compose up -d
open http://localhost:3001
```

### Can I run this on a Raspberry Pi?

Yes! Shroud LLM can run on a Raspberry Pi 4 (8GB recommended). Use lightweight local models like Phi or smaller Llama variants.

### How much RAM do I need?

- **Basic usage**: 4 GB
- **With small AI models**: 8 GB
- **With large AI models**: 16 GB+
- **Recommended**: 16 GB for best performance

### Is my data encrypted?

Yes! You can enable BYOK (Bring Your Own Keys) for zero-knowledge encryption where you control the encryption keys. Data is also encrypted at rest in the database.

---

## 🙏 Acknowledgments

Built with:
- [AnythingLLM](https://github.com/Mintplex-Labs/anything-llm) - Original foundation
- [React](https://react.dev/) - Frontend framework
- [Node.js](https://nodejs.org/) - Backend runtime
- [LanceDB](https://lancedb.com/) - Vector database
- [Ollama](https://ollama.ai/) - Local LLM runtime
- And many other amazing open source projects

Special thanks to:
- The Mintplex Labs team for creating AnythingLLM
- The open source community for contributions
- You, for choosing privacy! 🔒

---

<p align="center">
  <strong>🔒 Built with Privacy in Mind. Always. 🔒</strong>
</p>

<p align="center">
  <sub>Made with ❤️ by the Shadow Protocol team</sub>
</p>

<p align="center">
  <a href="#-quick-start">Get Started</a> •
  <a href="CONTRIBUTING.md">Contribute</a> •
  <a href="LICENSE">License</a> •
  <a href="https://github.com/lilbub1234/Shadow-Protocol">GitHub</a>
</p>
