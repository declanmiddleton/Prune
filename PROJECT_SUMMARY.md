# Prune - Project Summary

## What is Prune?

Prune is an **adaptive discovery engine** for penetration testing that intelligently discovers hidden directories, files, and subdomains. Unlike traditional brute-force tools, Prune learns from responses in real-time, automatically filters noise, and focuses on paths most likely to yield meaningful results.

## Key Innovations

### 1. Real-Time Intelligence
- Learns patterns from every response
- Automatically excludes uninformative status codes
- Detects and filters wildcard responses
- Adapts strategy based on discoveries

### 2. Pattern Learning
- Extracts successful path patterns
- Generates intelligent mutations
- Prioritizes words by confidence
- Learns naming conventions

### 3. Adaptive Rate Control
- Adjusts speed based on target responsiveness
- Prevents overwhelming slow targets
- Maximizes efficiency on fast targets
- Respects denial-of-service boundaries

### 4. Beautiful UI
- Calm, professional color scheme (#2596be blue, #5621d5 violet)
- Real-time progress with statistics
- Clear confidence indicators
- Intelligent feedback messages

## Technical Highlights

### Architecture
- **Language**: Rust
- **Async Runtime**: Tokio
- **Concurrency**: Futures + Semaphores
- **Data Structures**: DashMap for lock-free concurrency

### Intelligence Engine
- Status pattern tracking with DashMap
- Content-based wildcard detection
- Technology fingerprinting
- Confidence-based prioritization
- Continuous learning (no training phase)

### Discovery Modes
- **Directory**: Web path enumeration with mutations
- **Subdomain**: DNS-aware enumeration with HTTP probing
- **Combined**: Coordinated discovery with shared intelligence
- **Crawling**: Optional passive link extraction

## Project Structure

```
prune/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # Command-line interface
│   ├── engine.rs        # Discovery orchestration
│   ├── intelligence.rs  # Adaptive learning engine
│   ├── scanner.rs       # Directory enumeration
│   ├── subdomain.rs     # Subdomain discovery
│   ├── crawler.rs       # Link extraction
│   ├── session.rs       # State persistence
│   ├── wordlist.rs      # Wordlist management
│   ├── ui.rs            # Terminal interface
│   └── utils.rs         # Utilities
├── Cargo.toml           # Dependencies
├── README.md            # Main documentation
├── QUICKSTART.md        # Getting started guide
├── ARCHITECTURE.md      # Technical design
├── CONTRIBUTING.md      # Contribution guide
├── CHANGELOG.md         # Version history
├── LICENSE              # MIT License
└── examples.sh          # Usage examples
```

## Commands

```bash
prune scan <url>              # Interactive mode
prune dir <url>               # Directory discovery
prune sub <domain>            # Subdomain enumeration
prune both <url>              # Combined discovery
prune crawl on|off            # Toggle crawling
prune status exclude <codes>  # Exclude status codes
prune rate slow|normal|fast   # Set request rate
prune resume                  # Resume last session
prune results                 # Show findings
```

## Intelligence Features

### Pattern Recognition
- Successful path extraction
- Naming convention learning
- Technology-specific mutations
- Confidence scoring (0.0 - 1.0)

### Wildcard Detection
- Content signature hashing (SHA256)
- Size-based similarity
- Frequency analysis
- Automatic exclusion

### Technology Fingerprinting
- Web servers (nginx, Apache)
- Frameworks (WordPress, Laravel)
- Languages (PHP, Python)
- API patterns (REST, GraphQL)

### Adaptive Filtering
- Auto-exclude after 50+ identical responses
- Wildcard signature matching
- Uninformative status suppression
- Failing pattern deprioritization

## Performance

### Throughput
- **Default**: ~100 requests/second
- **Slow**: ~50 req/s (conservative)
- **Fast**: ~200 req/s (aggressive)
- **Adaptive**: Adjusts based on target

### Efficiency
- Prioritized wordlist processing
- Early filtering of noise
- Minimal unnecessary requests
- Intelligent mutation timing

### Scalability
- Concurrent async I/O
- Lock-free data structures
- Connection pooling
- Memory-efficient streaming

## Safety Features

### Target Protection
- Conservative default rates
- Adaptive slowdown for slow targets
- Timeout protection (10s default)
- DNS rate limiting (50% of HTTP rate)

### Scope Control
- Same-domain enforcement in crawler
- No automatic form submission
- Configurable boundaries
- Explicit permission model

### Network Safety
- Request timeouts
- Connection limits
- Graceful error handling
- Invalid cert support (opt-in)

## Use Cases

### Penetration Testing
- Comprehensive asset discovery
- Hidden path enumeration
- Technology stack identification
- Attack surface mapping

### Bug Bounty Hunting
- Efficient reconnaissance
- Low-noise scanning
- Session resumption for long scans
- Respectful rate limiting

### Security Audits
- Professional reporting
- Clean, filtered results
- Intelligence summaries
- Technology fingerprinting

## Comparison to Other Tools

### vs. Traditional Brute Force (gobuster, dirb)
- ✅ Learns and adapts
- ✅ Auto-filters noise
- ✅ Generates mutations
- ✅ Lower false positive rate

### vs. Smart Scanners (ffuf, feroxbuster)
- ✅ Combined directory + subdomain
- ✅ Real-time intelligence feedback
- ✅ Technology fingerprinting
- ✅ Session resumption
- ✅ Beautiful, calm UI

### Unique Features
- Confidence scoring for findings
- Pattern-based mutation generation
- Adaptive rate adjustment
- Intelligence summaries
- Seamless mode switching

## Dependencies

### Core
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `trust-dns-resolver` - DNS resolution
- `scraper` - HTML parsing

### Data Structures
- `dashmap` - Concurrent HashMap
- `parking_lot` - Fast locks

### CLI & UI
- `clap` - Command parsing
- `colored` - Terminal colors
- `crossterm` - Terminal control
- `indicatif` - Progress bars

### Utilities
- `serde` / `serde_json` - Serialization
- `chrono` - Timestamps
- `anyhow` - Error handling
- `regex` - Pattern matching

## Statistics

- **Lines of Code**: ~3,500
- **Modules**: 10
- **Default Directory Words**: 850+
- **Default Subdomain Words**: 500+
- **Compilation Time**: ~60s (release)
- **Binary Size**: ~8MB (stripped)

## Development Stats

- **Language**: 100% Rust
- **Unsafe Code**: 0%
- **Test Coverage**: Core modules
- **Build Warnings**: Minor unused functions
- **Compilation**: Zero errors

## Future Roadmap

### v0.2.0 (Planned)
- [ ] Machine learning wordlist optimization
- [ ] Export formats (JSON, CSV, XML)
- [ ] Enhanced mutation algorithms
- [ ] GraphQL endpoint discovery

### v0.3.0 (Planned)
- [ ] Distributed scanning support
- [ ] Plugin architecture
- [ ] Authentication-aware scanning
- [ ] Cloud function detection

### v1.0.0 (Planned)
- [ ] Full API fuzzing capabilities
- [ ] Advanced ML pattern recognition
- [ ] Professional report generation
- [ ] Commercial support options

## License

MIT License - Free for personal and commercial use

## Credits

Built with ❤️ using open-source Rust ecosystem

## Community

- **Repository**: github.com/yourusername/prune
- **Issues**: Report bugs and request features
- **Contributions**: See CONTRIBUTING.md
- **Security**: Responsible disclosure appreciated

## Philosophy

Prune embodies three principles:

1. **Intelligence over Brute Force** - Learn and adapt
2. **Signal over Noise** - Show what matters
3. **Beauty over Clutter** - Calm, clear interface

---

**Prune**: Trim the noise. Find what matters. 🌿
