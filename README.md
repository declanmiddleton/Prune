<div align="center">

<!-- 
  BANNER IMAGE SECTION
  Replace the path below with your custom banner image URL
  Recommended size: 1200x400px or similar aspect ratio
-->


<img width="1280" height="720" alt="Untitled design(1)" src="https://github.com/user-attachments/assets/94bf5b99-65b3-4828-8078-95728bf9de22" />


<!-- If hosting on GitHub, you can also use:
![Prune Banner](https://raw.githubusercontent.com/declanmiddleton/Prune/main/assets/banner.png)
-->

<h1>🌿 Prune</h1>

**Adaptive Discovery Engine for Modern Security Testing**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey.svg)]()

[Features](#-what-is-prune) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Documentation](#-documentation)

</div>

---

## 🎯 What is Prune?

**Prune is an intelligent web discovery tool that learns as it scans.** Instead of blindly brute-forcing thousands of paths, Prune adapts in real-time—filtering noise, detecting patterns, and focusing on what actually matters.

### Why Use Prune?

**Traditional tools** flood targets with static wordlists, producing thousands of useless 404s and overwhelming you with noise.

**Prune is different:**
- ✨ **Only Shows Valid Pages** - Automatically filters 404, 403, 429, 500, 502, 504 - you only see 200 OK responses
- 🎯 **SecLists Integration** - Automatically finds and uses SecLists for comprehensive coverage
- 🧠 **Learns From Responses** - Detects wildcards and uninformative patterns automatically
- 🔄 **Generates Mutations** - Creates intelligent path variations from discoveries
- 🚀 **Adapts Speed** - Adjusts request rate based on target responsiveness
- 🎨 **Beautiful Output** - Clear, color-coded results with confidence indicators
- 💾 **Session Management** - Resume long scans anytime

### Perfect For

- 🔍 **Penetration Testing** - Comprehensive attack surface discovery
- 🐛 **Bug Bounty Hunting** - Efficient reconnaissance with minimal noise
- 🛡️ **Security Audits** - Professional asset enumeration
- 🔬 **Research** - Technology fingerprinting and analysis

---

## ⚡ Features at a Glance

| Feature | Description |
|---------|-------------|
| **🎨 Graphical Interface** | Modern GUI with visual controls, progress tracking, and easy wordlist selection |
| **Adaptive Intelligence** | Real-time learning from status codes, content patterns, and response behavior |
| **SecLists Integration** | Automatically finds and uses SecLists wordlists (falls back to built-in 850+ dirs) |
| **Smart Filtering** | Only shows valid pages (200 OK) - automatically filters 404, 403, 429, 500, 502, 504 |
| **Directory Discovery** | Intelligent enumeration with comprehensive wordlists and pattern-based mutations |
| **Subdomain Enumeration** | DNS-aware discovery with 500+ common subdomains and intelligent variations |
| **Combined Mode** | Coordinate both techniques with shared intelligence |
| **Wildcard Detection** | Automatic identification and filtering of wildcard responses |
| **Technology Fingerprinting** | Detects web servers, frameworks, and tech stacks |
| **Passive Crawling** | Optional link extraction from discovered pages |
| **Session Persistence** | Save and resume scanning sessions (CLI mode) |
| **Rate Adaptation** | Dynamically adjusts from 50-200 req/s based on target |
| **Beautiful UI** | Professional GUI + terminal interface with calm color scheme |

---

## 📦 Installation

### Quick Install

```bash
# Clone the repository
git clone https://github.com/declanmiddleton/Prune.git
cd Prune

# Build the release binary
cargo build --release

# Run Prune
./target/release/prune --help
```

### System-Wide Installation

```bash
# Install using Cargo
cd Prune
cargo install --path .

# Now run from anywhere
prune --help
```

### Prerequisites

- **Rust 1.70+** ([Install Rust](https://rustup.rs/))
- **Cargo** (comes with Rust)

### Recommended: Install SecLists

Prune automatically detects and uses **SecLists** wordlists for better coverage:

```bash
# Clone SecLists to your home directory
git clone https://github.com/danielmiessler/SecLists.git ~/SecLists

# Or install to /usr/share (requires sudo)
sudo git clone https://github.com/danielmiessler/SecLists.git /usr/share/seclists
```

Prune will automatically find SecLists in common locations:
- `~/SecLists` or `~/seclists`
- `/usr/share/seclists`
- `/opt/SecLists`
- And more...

**Without SecLists:** Prune falls back to built-in curated wordlists (850 dirs + 500 subdomains)

---

## 🚀 Quick Start

### 0. Use the Graphical Interface (Easiest!)

For a visual experience with point-and-click simplicity:

```bash
prune gui
```

**Features:**
- 🎨 Beautiful modern interface
- 📁 Browse and select custom wordlists
- ⚙️ Configure all options visually
- 📊 Real-time progress tracking
- 💾 Export results to JSON/CSV

[Full GUI Guide →](GUI_GUIDE.md)

### 1. Your First CLI Scan

Discover hidden directories on a target:

```bash
prune dir https://example.com
```

**What happens:**
- Prune loads its intelligent wordlist
- Tests paths with adaptive concurrency
- Learns patterns and filters noise automatically
- Shows only meaningful findings

### 2. Find Subdomains

Enumerate subdomains with DNS awareness:

```bash
prune sub example.com
```

**Features:**
- DNS resolution before HTTP requests
- Smart naming pattern detection
- Automatic mutation generation
- Conservative rate limiting

### 3. Combined Discovery

Run both modes with shared intelligence:

```bash
prune both https://example.com
```

**Intelligence sharing:**
- Discoveries inform both scanners
- Patterns learned in one mode benefit the other
- Coordinated scanning for maximum efficiency

### 4. Interactive Mode

Let Prune guide you:

```bash
prune scan https://example.com
```

Choose your discovery mode interactively with helpful prompts.

---

## 📖 Usage

### Basic Commands

```bash
# Graphical interface (NEW!)
prune gui

# Interactive mode - prompts for scan type
prune scan <url>

# Directory discovery only
prune dir <url>

# Subdomain enumeration only
prune sub <domain>

# Combined discovery (recommended)
prune both <url>
```

### Configuration

```bash
# Enable passive crawling for deeper discovery
prune crawl on

# Set request rate (slow, normal, fast)
prune rate slow       # ~50 req/s (conservative)
prune rate normal     # ~100 req/s (default)
prune rate fast       # ~200 req/s (aggressive)

# Manually exclude noisy status codes
prune status exclude 404,502,503
```

### Session Management

```bash
# Resume your last scan
prune resume

# View results from last session
prune results
```

---

## 🎨 Example Output

```
 ____  ____  _  _  __ _  ____ 
(  _ \(  _ \/ )( \(  ( \(  __)
 ) __/ )   /) \/ (/    / ) _) 
(__)  (__\_)\____/\_)__)(____)

Adaptive Discovery Engine
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Directory Discovery
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ℹ Target: https://example.com
ℹ Session: prune_1736694120
✓ Found SecLists at: /home/user/SecLists
→ Using SecLists wordlist: common.txt
⚙ Loaded 4,713 words, prioritized by confidence
ℹ Starting scan with adaptive rate: 100 req/s

200 │ https://example.com/admin (5.2KB) ●●●
200 │ https://example.com/api (1.1KB) ●●●
200 │ https://example.com/login (3.4KB) ●●●
301 │ https://example.com/backup → /backups (0B) ●●○

⚙ Generated 12 adaptive mutations from successful patterns
⚙ Filtered out: 404, 403, 429, 500, 502, 504 (only showing valid pages)

→ ████████████████████████████░░░░░░░░░░░░ 65% │ 89.3 req/s │ 4 discoveries

Intelligence Summary
────────────────────────────────────────────────────────────
  Showing only: 200 OK, 301/302 redirects
  Excluded codes: [404, 403, 429, 500, 502, 504]
  Wildcard patterns: 2
  Generated mutations: 32
  Overall confidence: 82.3%

✓ Directory discovery complete!
```

### Understanding the Output

| Symbol | Meaning |
|--------|---------|
| `●●●` | High confidence (80%+) - Likely important |
| `●●○` | Medium confidence (50-80%) - Worth investigating |
| `●○○` | Low confidence (<50%) - Possibly interesting |
| `⚙` | Intelligence feedback - Prune is learning and adapting |

---

## 🧠 How Intelligence Works

Prune's adaptive engine learns from every response:

### 1. Pattern Recognition
- Tracks status code frequencies and patterns
- Identifies uninformative responses automatically
- Learns successful path structures

### 2. Wildcard Detection
- Calculates content signatures (SHA256)
- Detects identical responses with different paths
- Auto-excludes wildcard patterns

### 3. Technology Fingerprinting
- Identifies web servers (nginx, Apache, IIS)
- Detects frameworks (WordPress, Laravel, Django)
- Discovers API patterns (REST, GraphQL)

### 4. Smart Mutations
When Prune finds `/admin`, it intelligently generates:
- `/admin/login`, `/admin/dashboard`, `/admin/config`
- `/admin.bak`, `/admin.old`, `/admin~`
- `/api/admin`, `/v1/admin`

### 5. Adaptive Rate Control
- Slows down for slow/rate-limited targets
- Speeds up for fast-responding servers
- Prevents DoS conditions automatically

---

## 📁 Data Storage & Wordlists

### SecLists Integration (Recommended)

Prune automatically detects SecLists installation:

```bash
# Install SecLists for maximum coverage
git clone https://github.com/danielmiessler/SecLists.git ~/SecLists
```

**Wordlists used from SecLists:**
- `Discovery/Web-Content/common.txt` (~4,700 entries)
- `Discovery/Web-Content/directory-list-2.3-medium.txt` (~220,000 entries)
- `Discovery/DNS/subdomains-top1million-5000.txt` (5,000 entries)

### Local Storage

Prune stores sessions and config in `~/.prune/`:

```
~/.prune/
├── config.json              # User preferences
├── sessions/                # Saved scan sessions
│   ├── prune_1736694120.json
│   └── prune_1736694189.json
└── wordlists/               # Fallback wordlists (if SecLists not found)
    ├── directories.txt      # 850+ curated paths
    └── subdomains.txt       # 500+ common subdomains
```

### Custom Wordlists

If you prefer custom wordlists over SecLists:

```bash
# Add target-specific paths
nano ~/.prune/wordlists/directories.txt

# Add common subdomains for your industry
nano ~/.prune/wordlists/subdomains.txt
```

Prune will use custom wordlists if SecLists is not installed.

---

## 🛡️ Safety & Responsible Use

Prune is designed with safety in mind:

✅ **Conservative Defaults** - Safe rate limits out of the box  
✅ **Adaptive Throttling** - Automatically slows for struggling targets  
✅ **Request Timeouts** - Prevents hanging on slow endpoints  
✅ **No Destructive Actions** - Read-only operations  
✅ **Scope Enforcement** - Stays within target domain

### ⚠️ Legal Notice

**Only scan targets you have explicit permission to test.**

Unauthorized scanning may be illegal in your jurisdiction. Always:
- Obtain written permission before testing
- Respect scope and rules of engagement
- Follow responsible disclosure practices
- Stay within legal and ethical boundaries

---

## 📚 Documentation

- **[GUI Guide](GUI_GUIDE.md)** - Complete graphical interface tutorial
- **[Quick Start Guide](QUICKSTART.md)** - Get up and running in 5 minutes
- **[Architecture Overview](ARCHITECTURE.md)** - Technical design and internals
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute to Prune
- **[Changelog](CHANGELOG.md)** - Version history and updates

---

## 🔧 Advanced Configuration

### Configuration File

Edit `~/.prune/config.json`:

```json
{
  "crawling_enabled": false,
  "rate_limit": 100,
  "excluded_status_codes": [404, 403, 429, 500, 502, 504, 405, 501],
  "max_depth": 3,
  "timeout_seconds": 10
}
```

**Default behavior:** Only shows status 200 (OK) and 301/302 (redirects)  
**Filtered by default:** 404, 403, 429, 500, 502, 504 and other error codes

### Environment Tuning

```bash
# Maximum performance (use with caution)
prune rate fast
prune crawl on

# Stealth mode (slow and quiet)
prune rate slow
prune crawl off

# Balanced (recommended for most scenarios)
prune rate normal
prune crawl off
```

---

## 🤝 Contributing

Contributions are welcome! Whether it's:

- 🐛 Bug reports and fixes
- ✨ New features and enhancements
- 📖 Documentation improvements
- 💡 Ideas and suggestions

Please read our [Contributing Guide](CONTRIBUTING.md) to get started.

---

## 🗺️ Roadmap

### v0.2.0 (Planned)
- [ ] Machine learning-based wordlist optimization
- [ ] Enhanced mutation algorithms
- [ ] GraphQL endpoint discovery
- [ ] Export formats (JSON, CSV, XML)

### v0.3.0 (Planned)
- [ ] Distributed scanning support
- [ ] Plugin architecture
- [ ] Authentication-aware scanning
- [ ] Cloud function detection

### v1.0.0 (Future)
- [ ] Full API fuzzing capabilities
- [ ] Advanced ML pattern recognition
- [ ] Professional report generation
- [ ] Commercial support options

---

## 💬 Support & Community

- **Issues**: [Report bugs or request features](https://github.com/declanmiddleton/Prune/issues)
- **Discussions**: [Ask questions and share ideas](https://github.com/declanmiddleton/Prune/discussions)
- **Security**: Found a vulnerability? Please report responsibly

---

## 📊 Statistics

| Metric | Value |
|--------|-------|
| **Lines of Code** | 3,500+ |
| **Dependencies** | 25+ Rust crates |
| **Binary Size** | 6.1 MB (optimized) |
| **Default Wordlists** | 1,350+ entries |
| **Request Rate** | 50-200 req/s (adaptive) |
| **Concurrency Model** | Async with Tokio |

---

## 🙏 Acknowledgments

Built with these amazing open-source projects:

- [Tokio](https://tokio.rs/) - Async runtime
- [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [Trust-DNS](https://github.com/bluejekyll/trust-dns) - DNS resolution
- [Clap](https://github.com/clap-rs/clap) - CLI parsing
- [Colored](https://github.com/mackwic/colored) - Terminal colors

And the entire Rust community for creating such a powerful language! 🦀

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

```
MIT License - Copyright (c) 2026 Declan Middleton
```

---

## ⭐ Show Your Support

If Prune helped you discover something interesting, consider:

- ⭐ **Starring this repository**
- 🐛 **Reporting bugs** to help improve the tool
- 💡 **Sharing your experience** with the community
- 🤝 **Contributing** new features or improvements

---

<div align="center">

**Made with 💜 by [Declan Middleton](https://github.com/declanmiddleton)**

*"Trim the noise. Find what matters."* 🌿

[⬆ Back to Top](#-prune)

</div>
