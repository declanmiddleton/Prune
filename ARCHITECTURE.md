# Prune Architecture

This document describes the internal architecture and design decisions of Prune.

## Overview

Prune is built as a modular, async Rust application that coordinates multiple discovery techniques with a shared intelligence layer. The architecture prioritizes adaptability, efficiency, and user experience.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│  (Command parsing, user interaction, configuration)          │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                    Discovery Engine                          │
│  (Orchestrates scanning modes and phases)                    │
└────┬──────────────┬──────────────┬────────────┬────────────┘
     │              │              │            │
┌────▼────┐  ┌─────▼─────┐  ┌─────▼─────┐  ┌──▼──────────┐
│Directory│  │ Subdomain │  │  Crawler  │  │   Session   │
│ Scanner │  │Enumerator │  │           │  │  Manager    │
└────┬────┘  └─────┬─────┘  └─────┬─────┘  └──┬──────────┘
     │              │              │            │
     └──────────────┴──────────────┴────────────┘
                      │
     ┌────────────────▼────────────────┐
     │   Intelligence Engine           │
     │ (Pattern learning, filtering)   │
     └────────────────┬────────────────┘
                      │
     ┌────────────────▼────────────────┐
     │      Wordlist Manager           │
     │  (Load and manage wordlists)    │
     └─────────────────────────────────┘
```

## Core Components

### 1. CLI Layer (`cli.rs`)

**Responsibility**: User interface and command routing

**Key Functions**:
- Parse command-line arguments with `clap`
- Route to appropriate handlers
- Present interactive prompts
- Format and display results

**Design Decisions**:
- Uses `clap` derive API for declarative command definition
- Each command has a dedicated handler function
- Separates parsing from execution logic

### 2. Discovery Engine (`engine.rs`)

**Responsibility**: Orchestrate discovery operations

**Key Functions**:
- Initialize HTTP client and intelligence engine
- Coordinate directory scanning
- Coordinate subdomain enumeration
- Manage combined discovery mode
- Handle session resumption

**Design Decisions**:
- Single engine instance per scan
- Shared intelligence across all operations
- Async architecture with Tokio runtime
- Arc-based sharing for thread safety

### 3. Intelligence Engine (`intelligence.rs`)

**Responsibility**: Learn from responses and adapt strategy

**Key Features**:
- **Status Pattern Tracking**: Monitor frequency and informative value
- **Wildcard Detection**: Identify and filter repeated responses
- **Pattern Learning**: Extract and score successful patterns
- **Technology Fingerprinting**: Detect frameworks and servers
- **Adaptive Rate Control**: Adjust speed based on responsiveness

**Data Structures**:
```rust
DashMap<u16, StatusPattern>     // Concurrent status tracking
RwLock<HashSet<String>>         // Wildcard signatures
DashMap<String, PatternScore>   // Pattern confidence scores
RwLock<RequestStats>            // Performance metrics
```

**Design Decisions**:
- Lock-free concurrent data structures (`DashMap`)
- Reader-writer locks for shared state
- Probabilistic filtering to minimize false positives
- Continuous learning with no training phase

### 4. Directory Scanner (`scanner.rs`)

**Responsibility**: Enumerate web directories and files

**Key Features**:
- Prioritized wordlist processing
- Concurrent request handling with rate limiting
- Adaptive mutation generation
- Optional crawling integration
- Real-time progress feedback

**Flow**:
1. Load wordlist from manager
2. Prioritize by intelligence confidence
3. Process in chunks with concurrency
4. Learn from each response
5. Generate mutations from successes
6. Optionally crawl discovered paths

**Design Decisions**:
- Chunk-based processing (50 words) for adaptability
- Semaphore-based rate limiting
- Futures-based concurrency with `buffer_unordered`
- Progressive mutation injection

### 5. Subdomain Enumerator (`subdomain.rs`)

**Responsibility**: Discover subdomains via DNS and HTTP

**Key Features**:
- DNS resolution with trust-dns
- HTTP/HTTPS probing
- Pattern-based mutation
- Conservative rate limiting
- Intelligence-guided prioritization

**Flow**:
1. Load subdomain wordlist
2. Prioritize by confidence
3. DNS lookup for each candidate
4. HTTP/HTTPS probe for resolved domains
5. Learn and generate mutations
6. Filter with intelligence

**Design Decisions**:
- Two-phase approach (DNS then HTTP)
- Lower rate limit than directory scanning
- Try HTTPS before HTTP
- Timeout protection for slow DNS

### 6. Crawler (`crawler.rs`)

**Responsibility**: Extract additional paths from responses

**Key Features**:
- Parse HTML for links
- Extract from scripts, forms, comments
- URL pattern extraction with regex
- Scope enforcement
- Strict rate limiting

**Sources**:
- `<a href>` tags
- `<script src>` attributes
- `<form action>` attributes
- Inline JavaScript URLs
- HTML comments

**Design Decisions**:
- Passive extraction only (no form submission)
- Strict scope checking (same domain)
- File extension filtering (no images, media)
- Crawl limit to prevent explosion
- Visit tracking to avoid loops

### 7. Session Manager (`session.rs`)

**Responsibility**: Persist and restore scan state

**Key Features**:
- Save results to disk
- Load previous sessions
- Store configuration
- Support resume operations

**Storage Location**: `~/.prune/`
```
~/.prune/
├── config.json
├── sessions/
│   ├── prune_1234567890.json
│   └── prune_1234567891.json
└── wordlists/
    ├── directories.txt
    └── subdomains.txt
```

**Design Decisions**:
- JSON format for human readability
- Timestamp-based session IDs
- Home directory storage for user isolation
- Separate config from session data

### 8. Wordlist Manager (`wordlist.rs`)

**Responsibility**: Load and manage discovery wordlists

**Key Features**:
- Initialize default wordlists
- Load custom wordlists
- Filter comments and empty lines
- Separate lists for directories and subdomains

**Default Wordlists**:
- **directories.txt**: 850+ curated paths
- **subdomains.txt**: 500+ common subdomains

**Design Decisions**:
- Embedded defaults in binary
- User-customizable via `~/.prune/wordlists/`
- Plain text format for easy editing
- Focus on quality over quantity

### 9. UI Module (`ui.rs`)

**Responsibility**: Terminal interface and visual feedback

**Key Features**:
- Beautiful ASCII banner
- Color-coded status messages
- Progress bars with statistics
- Finding display with confidence
- Intelligence summaries

**Color Scheme**:
- Primary: `#2596be` (Blue) - Actions, discoveries
- Secondary: `#5621d5` (Violet) - Metadata, context
- Success: Blue
- Warning: Orange
- Error: Red

**Design Decisions**:
- True color support with `truecolor()` method
- Unicode characters for visual appeal
- Inline progress updates (no screen clearing)
- Calm, professional aesthetic

## Data Flow

### Directory Discovery Flow

```
User Input → CLI → Engine → Scanner
                             │
                             ├→ Load Wordlist
                             ├→ Prioritize via Intelligence
                             ├→ Concurrent Requests
                             │   └→ Learn from Response
                             ├→ Generate Mutations
                             ├→ Optional Crawling
                             └→ Return Results
                                  │
                                  ├→ Display in UI
                                  └→ Save to Session
```

### Intelligence Learning Flow

```
Response → Extract Features
           ├→ Status Code
           ├→ Content Size
           ├→ Body Hash
           ├→ Headers
           └→ Path Components
                │
                ├→ Update Status Patterns
                ├→ Detect Wildcards
                ├→ Learn Success Patterns
                ├→ Track Failures
                ├→ Fingerprint Tech
                └→ Adjust Confidence
```

## Concurrency Model

### Async Runtime
- **Framework**: Tokio
- **Pattern**: Async/await with futures
- **Concurrency**: Semaphore-based rate limiting

### Thread Safety
- **Shared State**: `Arc` + `RwLock` or `DashMap`
- **Pattern**: Lock-free when possible
- **Contention**: Minimized via chunk processing

### Rate Limiting
```rust
Semaphore(max_concurrent)
    ↓
Acquire permit before request
    ↓
Release permit after response
    ↓
Adaptive adjustment based on performance
```

## Performance Optimizations

1. **Prioritized Processing**: High-confidence words first
2. **Chunked Execution**: Process in batches for adaptability
3. **Concurrent I/O**: Multiple requests in flight
4. **Early Filtering**: Reject obvious wildcards immediately
5. **Minimal Allocations**: Reuse HTTP client, use streaming
6. **Smart Mutations**: Only generate when discoveries occur

## Security Considerations

### Input Validation
- URL parsing with `url` crate
- Domain validation before DNS lookup
- Path sanitization for filesystem operations

### Rate Limiting
- Default limits prevent abuse
- Adaptive reduction for slow targets
- Conservative DNS rates

### Scope Enforcement
- Same-domain checking in crawler
- No automatic form submission
- Configurable boundaries

### Network Safety
- Timeout on all requests
- Invalid certificate acceptance (opt-in for pentesting)
- Connection reuse with HTTP/2

## Testing Strategy

### Unit Tests
- Intelligence pattern detection
- URL normalization
- Confidence calculations

### Integration Tests
- Full scan workflows
- Session persistence
- Configuration loading

### Manual Testing
- Real-world targets (with permission)
- Edge cases (wildcards, redirects)
- Performance profiling

## Future Architecture Considerations

### Potential Enhancements
1. **Plugin System**: Allow custom intelligence modules
2. **Distributed Scanning**: Coordinate multiple instances
3. **ML Integration**: Deep learning for pattern recognition
4. **GraphQL Support**: Schema enumeration
5. **API Fuzzing**: Parameter discovery and testing

### Scalability
- Current design handles 100-200 req/s comfortably
- Could scale to 1000+ req/s with connection pooling
- Memory usage grows linearly with results

### Extensibility
- Modular design allows easy component replacement
- Intelligence engine can be swapped or enhanced
- New discovery modes can be added alongside existing ones

---

This architecture balances sophistication with simplicity, enabling powerful adaptive discovery while maintaining clean, maintainable code.
