# Prune 🌿

```
 ____  ____  _  _  __ _  ____ 
(  _ \(  _ \/ )( \(  ( \(  __)
 ) __/ )   /) \/ (/    / ) _) 
(__)  (__\_)\____/\_)__)(____)
```

**Adaptive Discovery Engine for Penetration Testing**

Prune is an intelligent discovery tool that combines directory scanning, subdomain enumeration, and optional crawling into a single adaptive workflow. Instead of brute-forcing static wordlists, it learns from live responses, filters noise automatically, and prioritizes requests that are most likely to produce results.

## 🎯 Features

### Intelligent Adaptation
- **Real-time Learning**: Observes response patterns and automatically excludes uninformative status codes
- **Wildcard Detection**: Identifies and filters wildcard responses to reduce noise
- **Pattern Recognition**: Learns from successful discoveries to generate intelligent mutations
- **Technology Fingerprinting**: Detects technologies and frameworks to guide discovery
- **Adaptive Rate Limiting**: Dynamically adjusts request rate based on target responsiveness

### Discovery Modes
- **Directory Discovery**: Adaptive path enumeration with intelligent mutation
- **Subdomain Enumeration**: DNS-aware subdomain discovery with pattern learning
- **Combined Mode**: Coordinates both techniques, sharing intelligence between phases
- **Optional Crawling**: Passive link extraction within strict rate limits

### User Experience
- **Beautiful Terminal UI**: Calm, professional interface with blue/violet color scheme
- **Real-time Feedback**: Live progress bars, discovery notifications, and adaptation indicators
- **Session Persistence**: Save and resume scanning sessions
- **Minimal Noise**: Only shows meaningful findings, filtered automatically

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/prune
cd prune

# Build with Cargo
cargo build --release

# Install to system
cargo install --path .
```

## 📖 Usage

### Basic Commands

```bash
# Interactive mode - prompts for scan type
prune scan https://example.com

# Directory discovery only
prune dir https://example.com

# Subdomain enumeration only
prune sub example.com

# Both modes with shared intelligence
prune both https://example.com
```

### Configuration

```bash
# Enable passive crawling
prune crawl on

# Disable crawling
prune crawl off

# Manually exclude status codes
prune status exclude 404,503,500

# Set request rate limit
prune rate slow     # ~50 req/s
prune rate normal   # ~100 req/s (default)
prune rate fast     # ~200 req/s
```

### Session Management

```bash
# Resume last session
prune resume

# View results from last session
prune results
```

## 🧠 How Intelligence Works

### Response Learning
Prune learns from every response:
- Tracks status code patterns and frequencies
- Calculates average response sizes
- Identifies wildcard behaviors
- Detects uninformative responses

### Pattern Recognition
When discoveries are made:
- Extracts successful path patterns
- Learns naming conventions
- Generates intelligent mutations
- Prioritizes similar words

### Adaptive Filtering
Automatically excludes:
- Wildcard responses (same size/content)
- Uninformative status codes (seen >50 times)
- Consistently failing patterns
- Out-of-scope resources

### Technology Fingerprinting
Detects technologies from headers and body:
- Web servers (nginx, Apache)
- Frameworks (Laravel, WordPress)
- Languages (PHP, Python)
- API patterns

### Mutation Generation
Creates intelligent variations:
- Based on successful patterns
- Common backup/old file patterns
- API versioning (v1, v2, etc.)
- Technology-specific paths

## 🎨 Visual Design

Prune uses a consistent two-color system:

- **Primary Color** (`#2596be` - Blue): Active prompts, progress bars, discoveries
- **Secondary Color** (`#5621d5` - Violet): Metadata, status info, intelligence updates
- **Accent**: Lighter tints for subtle feedback
- **Warnings**: Muted yellow/red (used sparingly)

## 📁 Data Storage

Prune stores data in `~/.prune/`:

```
~/.prune/
├── config.json           # User configuration
├── sessions/             # Saved scan sessions
│   ├── prune_1234567890.json
│   └── prune_1234567891.json
└── wordlists/            # Discovery wordlists
    ├── directories.txt   # Directory wordlist
    └── subdomains.txt    # Subdomain wordlist
```

## 🛡️ Safety Features

- **Conservative DNS Rate Limiting**: Prevents DNS abuse
- **Adaptive Request Pacing**: Slows down when target is slow
- **Timeout Protection**: Prevents hanging on slow targets
- **Scope Enforcement**: Stays within target domain
- **Crawl Limits**: Strict limits on crawled pages

## 🔧 Advanced Configuration

### Custom Wordlists

Replace default wordlists in `~/.prune/wordlists/`:

```bash
# Custom directory wordlist
~/.prune/wordlists/directories.txt

# Custom subdomain wordlist
~/.prune/wordlists/subdomains.txt
```

### Configuration File

Edit `~/.prune/config.json`:

```json
{
  "crawling_enabled": false,
  "rate_limit": 100,
  "excluded_status_codes": [404, 405, 501],
  "max_depth": 3,
  "timeout_seconds": 10
}
```

## 🎯 Use Cases

### Penetration Testing
- Discover hidden directories and files
- Enumerate subdomains for expanded attack surface
- Find backup files and development resources
- Identify API endpoints and versions

### Bug Bounty Hunting
- Efficient reconnaissance with minimal noise
- Adaptive scanning that respects rate limits
- Technology fingerprinting for targeted testing
- Session resumption for long-running scans

### Security Audits
- Comprehensive asset discovery
- Technology stack identification
- Wildcard and virtual host detection
- Clean, professional reporting

## 🤝 Best Practices

1. **Start Conservative**: Use `normal` or `slow` rate limits initially
2. **Enable Crawling Selectively**: Only when you need deeper discovery
3. **Review Intelligence**: Check excluded codes and wildcards
4. **Resume Long Scans**: Use `prune resume` for interrupted scans
5. **Respect Scope**: Only scan targets you have permission to test

## 📊 Example Output

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
ℹ Session: prune_1234567890
ℹ Loading wordlist...
⚙ Loaded 850 words, prioritized by confidence
ℹ Starting scan with adaptive rate: 100 req/s

200 │ https://example.com/admin (5.2KB) ●●●
200 │ https://example.com/api/v1 (1.1KB) ●●●
301 │ https://example.com/old (0B) ●●○
403 │ https://example.com/config (0B) ●○○

⚙ Generated 8 adaptive mutations from successful patterns
⚙ Adapting rate limit: 100 → 75 req/s

→ ████████████████████████████░░░░░░░░░░░░ 65% │ 89.3 req/s │ 4 discoveries

Intelligence Summary
────────────────────────────────────────────────────────────
  Excluded codes: [404, 405, 502]
  Wildcard patterns: 2
  Generated mutations: 24
  Overall confidence: 78.5%

✓ Directory discovery complete!
```

## 🐛 Troubleshooting

### No Results Found
- Check if target is reachable
- Verify URL format includes scheme (http:// or https://)
- Try lowering rate limit: `prune rate slow`
- Check excluded status codes in config

### Too Many False Positives
- Intelligence is still learning (wait for ~50 requests)
- Manually exclude status codes: `prune status exclude 200`
- Review wildcard detection in intelligence summary

### DNS Errors (Subdomain Mode)
- Check DNS resolver configuration
- Try reducing rate: `prune rate slow`
- Verify domain is valid and resolvable

## 📝 License

MIT License - See LICENSE file for details

## 🙏 Credits

Built with:
- [tokio](https://tokio.rs/) - Async runtime
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [clap](https://github.com/clap-rs/clap) - CLI parsing
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [trust-dns](https://github.com/bluejekyll/trust-dns) - DNS resolution
- [scraper](https://github.com/causal-agent/scraper) - HTML parsing

## 🔮 Roadmap

- [ ] Machine learning-based wordlist optimization
- [ ] Cloud function and serverless detection
- [ ] API schema discovery and fuzzing
- [ ] GraphQL endpoint enumeration
- [ ] Authentication-aware scanning
- [ ] Distributed scanning support
- [ ] Export to common formats (JSON, CSV, XML)

---

**Made with 💜 for the security community**

*Remember: Only scan targets you have explicit permission to test.*
