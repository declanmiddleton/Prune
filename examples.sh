#!/bin/bash
# Example usage of Prune discovery tool

# Make sure Prune is built
if [ ! -f "./target/release/prune" ]; then
    echo "Building Prune..."
    cargo build --release
fi

PRUNE="./target/release/prune"

echo "==================================="
echo "Prune Discovery Tool - Examples"
echo "==================================="
echo ""

# Show help
echo "1. Show help:"
echo "   $PRUNE --help"
echo ""

# Directory scan example
echo "2. Directory discovery:"
echo "   $PRUNE dir https://example.com"
echo ""

# Subdomain enumeration example
echo "3. Subdomain enumeration:"
echo "   $PRUNE sub example.com"
echo ""

# Combined scan example
echo "4. Combined discovery:"
echo "   $PRUNE both https://example.com"
echo ""

# Interactive scan
echo "5. Interactive mode:"
echo "   $PRUNE scan https://example.com"
echo ""

# Configuration examples
echo "6. Enable crawling:"
echo "   $PRUNE crawl on"
echo ""

echo "7. Set rate limit:"
echo "   $PRUNE rate slow"
echo ""

echo "8. Exclude status codes:"
echo "   $PRUNE status exclude 404,500,502"
echo ""

# Session management
echo "9. Resume previous session:"
echo "   $PRUNE resume"
echo ""

echo "10. View results:"
echo "    $PRUNE results"
echo ""

echo "==================================="
echo "For a real test (safe target):"
echo "==================================="
echo "$PRUNE dir http://testphp.vulnweb.com"
echo ""
