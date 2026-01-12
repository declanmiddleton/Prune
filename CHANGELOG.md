# Changelog

All notable changes to Prune will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-12

### Added
- Initial release of Prune adaptive discovery engine
- Intelligent directory scanning with pattern learning
- Adaptive subdomain enumeration with DNS awareness
- Real-time response learning and filtering
- Wildcard detection and automatic exclusion
- Technology fingerprinting (nginx, Apache, PHP, WordPress, etc.)
- Pattern-based mutation generation
- Optional passive web crawling with rate limits
- Beautiful terminal UI with blue/violet color scheme
- Session persistence and resume capability
- Configurable rate limiting (slow/normal/fast)
- Manual status code exclusion
- Combined discovery mode with intelligence sharing
- Default curated wordlists for directories and subdomains
- Adaptive rate adjustment based on target responsiveness
- Progress tracking with real-time statistics
- Intelligence summaries showing learned patterns

### Features
- Interactive mode with scan type selection
- Directory-only discovery mode
- Subdomain-only enumeration mode
- Combined discovery mode
- Crawling enable/disable configuration
- Custom status code exclusion
- Configurable request pacing
- Session resumption
- Results viewing

### Intelligence Capabilities
- Status code pattern tracking
- Response size analysis
- Wildcard signature detection
- Successful pattern extraction
- Failing pattern deprioritization
- Technology stack detection
- Naming convention learning
- Adaptive wordlist prioritization
- Intelligent mutation generation
- Dynamic rate limiting

### Safety Features
- Conservative DNS rate limiting
- Adaptive request pacing
- Timeout protection
- Scope enforcement
- Strict crawl limits
- Invalid certificate acceptance (for pentesting)

### Documentation
- Comprehensive README with usage examples
- Contributing guidelines
- MIT License
- Example usage scripts
- Inline code documentation

[0.1.0]: https://github.com/yourusername/prune/releases/tag/v0.1.0
