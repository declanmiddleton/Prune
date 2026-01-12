# Prune Quick Start Guide

Get started with Prune in 5 minutes!

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/prune
cd prune

# Build the release binary
cargo build --release

# Optional: Add to PATH
export PATH=$PATH:$(pwd)/target/release
```

## Your First Scan

### 1. Directory Discovery

Scan a website for hidden directories and files:

```bash
prune dir https://example.com
```

**What happens:**
- Prune loads its directory wordlist
- Prioritizes words based on learned patterns
- Tests each path adaptively
- Filters out noise automatically
- Shows only meaningful findings

**Example output:**
```
200 │ https://example.com/admin (5.2KB) ●●●
403 │ https://example.com/config (0B) ●●○
301 │ https://example.com/old → /new (0B) ●○○
```

### 2. Subdomain Enumeration

Find subdomains of a domain:

```bash
prune sub example.com
```

**What happens:**
- Prune performs DNS lookups
- Tests discovered subdomains via HTTP/HTTPS
- Learns naming patterns
- Generates intelligent variations
- Filters dead ends

### 3. Combined Discovery

Run both modes with shared intelligence:

```bash
prune both https://example.com
```

**What happens:**
1. Discovers subdomains
2. Scans main target
3. Optionally scans discovered subdomains
4. Shares intelligence between phases

## Understanding the Output

### Status Codes
- **Blue (200-299)**: Success - resource found
- **Violet (300-399)**: Redirect - follow or investigate
- **Yellow (400-499)**: Client error - may still be interesting (403, 401)
- **Red (500-599)**: Server error

### Confidence Indicators
- **●●●** High confidence (80%+) - very likely important
- **●●○** Medium confidence (50-80%) - worth investigating
- **●○○** Low confidence (<50%) - possibly interesting

### Intelligence Feedback
Watch for these messages:
```
⚙ Loaded 850 words, prioritized by confidence
⚙ Generated 8 adaptive mutations from successful patterns
⚙ Adapting rate limit: 100 → 75 req/s
```

These show Prune learning and adapting in real-time!

## Basic Configuration

### Enable Crawling

Crawl discovered pages for additional paths:

```bash
prune crawl on
```

Now directory scans will also extract links from successful responses.

### Adjust Speed

```bash
# Conservative (recommended for most targets)
prune rate slow      # ~50 req/s

# Balanced (default)
prune rate normal    # ~100 req/s

# Aggressive (only for resilient targets)
prune rate fast      # ~200 req/s
```

### Exclude Noisy Status Codes

```bash
prune status exclude 404,502,503
```

## Session Management

### Resume Interrupted Scan

```bash
# If a scan is interrupted (Ctrl+C)
prune resume
```

Prune will continue from where it left off using saved session data.

### View Results

```bash
prune results
```

Shows findings from the last completed scan.

## Interactive Mode

Not sure which mode to use?

```bash
prune scan https://example.com
```

Prune will ask:
```
Select discovery mode:
  1. Directory discovery only
  2. Subdomain enumeration only
  3. Both (combined intelligence)

→ 
```

## Understanding Intelligence

### How Prune Learns

**From Status Codes:**
- Tracks frequency and patterns
- Identifies uninformative codes
- Auto-excludes after ~50 repetitions

**From Content:**
- Detects wildcard responses
- Calculates content signatures
- Filters duplicate patterns

**From Success:**
- Extracts path components
- Learns naming conventions
- Generates similar paths

**From Technology:**
- Detects web servers
- Identifies frameworks
- Suggests relevant paths

### Intelligence Summary

After each scan, review:
```
Intelligence Summary
────────────────────────────────────────────
  Excluded codes: [404, 502]
  Wildcard patterns: 2
  Generated mutations: 24
  Overall confidence: 78.5%
```

**High confidence** (>70%) = Prune is learning well
**Low confidence** (<50%) = Target may have unusual behavior

## Common Patterns

### Bug Bounty Recon
```bash
# Phase 1: Find subdomains
prune sub target.com

# Phase 2: Scan main target deeply
prune crawl on
prune dir https://target.com

# Phase 3: Scan interesting subdomains
prune dir https://api.target.com
prune dir https://admin.target.com
```

### Pentest Enumeration
```bash
# Comprehensive scan
prune both https://target.internal

# Enable everything
prune crawl on
prune rate normal

# Resume if interrupted
prune resume
```

### Quiet Reconnaissance
```bash
# Very conservative
prune rate slow
prune crawl off
prune dir https://target.com
```

## Custom Wordlists

Prune stores wordlists in `~/.prune/wordlists/`:

```bash
# Edit directory wordlist
nano ~/.prune/wordlists/directories.txt

# Edit subdomain wordlist  
nano ~/.prune/wordlists/subdomains.txt
```

Add target-specific words at the top for priority.

## Tips for Best Results

1. **Start conservative**: Use `slow` rate initially
2. **Let it learn**: Wait for 100+ requests before judging
3. **Review intelligence**: Check excluded codes make sense
4. **Enable crawling selectively**: Only when you need depth
5. **Use combined mode**: For comprehensive discovery
6. **Save sessions**: Long scans can be resumed

## Troubleshooting

### No results found
- Verify target is reachable: `curl https://example.com`
- Check rate isn't too high: `prune rate slow`
- Review excluded codes: look at intelligence summary

### Too many false positives
- Wait for intelligence to learn (~50 requests)
- Manually exclude noisy codes
- Check if target uses wildcards

### Scan is slow
- Increase rate: `prune rate fast`
- Target may be slow/rate-limiting
- Check your network connection

## Next Steps

- Read the full [README.md](README.md)
- Explore [CONTRIBUTING.md](CONTRIBUTING.md)
- Check [examples.sh](examples.sh) for more use cases
- Customize wordlists for your targets

## Safety Reminder

⚠️ **Only scan targets you have explicit permission to test!**

Prune is a powerful tool. Use it responsibly and ethically.

---

Happy hunting! 🌿
