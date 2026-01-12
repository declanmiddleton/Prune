# Recent Changes - SecLists Integration & Smart Filtering

## 🎯 Summary

Prune now automatically uses **SecLists** wordlists and **only shows successful responses** (200 OK), filtering out all error codes by default.

---

## ✨ What Changed

### 1. SecLists Auto-Detection

**Prune now automatically finds and uses SecLists** from common installation locations:

```
✓ Checks ~/SecLists or ~/seclists
✓ Checks /usr/share/seclists
✓ Checks /opt/SecLists
✓ Checks ~/wordlists/seclists
✓ Checks ~/Tools/SecLists
```

**When found, Prune uses:**
- `Discovery/Web-Content/common.txt` (~4,700 paths)
- `Discovery/Web-Content/directory-list-2.3-medium.txt` (~220,000 paths)
- `Discovery/DNS/subdomains-top1million-5000.txt` (5,000 subdomains)

**If SecLists is not found:**
- Falls back to built-in curated wordlists (850 dirs + 500 subdomains)
- Shows a helpful message on how to install SecLists

### 2. Smart Status Code Filtering

**Now filters by default:**
- ❌ 404 - Not Found
- ❌ 403 - Forbidden
- ❌ 429 - Too Many Requests
- ❌ 500 - Internal Server Error
- ❌ 502 - Bad Gateway
- ❌ 504 - Gateway Timeout
- ❌ 405 - Method Not Allowed
- ❌ 501 - Not Implemented

**Only shows:**
- ✅ 200 - OK (valid pages)
- ✅ 301 - Permanent Redirect (interesting)
- ✅ 302 - Temporary Redirect (interesting)

### 3. Updated Intelligence Engine

The intelligence engine now:
- Considers only 200-299 status codes as successful
- Includes 301/302 redirects as interesting findings
- Automatically excludes all error codes
- Focuses learning on valid responses only

---

## 📦 How to Install SecLists

### Option 1: Home Directory (Recommended)

```bash
cd ~
git clone https://github.com/danielmiessler/SecLists.git
```

### Option 2: System-Wide

```bash
sudo git clone https://github.com/danielmiessler/SecLists.git /usr/share/seclists
```

### Option 3: Custom Location

```bash
git clone https://github.com/danielmiessler/SecLists.git /path/to/SecLists
```

Prune will find it automatically in most common locations!

---

## 🚀 What This Means for You

### Before (without these changes):
```
Testing 850 paths...
404 │ https://example.com/test1
404 │ https://example.com/test2
403 │ https://example.com/admin
404 │ https://example.com/test3
500 │ https://example.com/error
200 │ https://example.com/login
404 │ https://example.com/test4
...hundreds more 404s...
```

### After (with these changes):
```
✓ Found SecLists at: /home/user/SecLists
→ Using SecLists wordlist: common.txt
Testing 4,713 paths...

200 │ https://example.com/login (3.2KB) ●●●
200 │ https://example.com/admin (5.1KB) ●●●
301 │ https://example.com/backup → /backups ●●○

⚙ Filtered out: 404, 403, 429, 500, 502, 504
→ Only showing valid pages (200 OK)
```

**Benefits:**
1. **Cleaner Output** - No more walls of 404s
2. **Better Coverage** - Using industry-standard SecLists
3. **Faster Analysis** - See only what matters
4. **Actionable Results** - Every result is worth investigating

---

## 🎛️ Customization

### To Include Additional Status Codes

If you want to see 403s or other codes:

```bash
# Edit config to remove codes from exclusion list
nano ~/.prune/config.json

# Or use command line
prune status exclude 404,429,500,502,504
# (This will now ONLY exclude these, showing 403)
```

### To Use Custom Wordlists Instead of SecLists

Simply don't install SecLists, and Prune will use built-in wordlists:

```bash
# Edit custom wordlists
nano ~/.prune/wordlists/directories.txt
nano ~/.prune/wordlists/subdomains.txt
```

### To Force Built-in Wordlists Even with SecLists

Move or rename your SecLists directory temporarily:

```bash
mv ~/SecLists ~/SecLists.bak
```

---

## 📊 Comparison

| Feature | Before | After |
|---------|--------|-------|
| **Default Wordlist** | Built-in 850 paths | SecLists 4,700+ paths |
| **Status Filtering** | Shows 404, 403, 500 | Only shows 200, 301, 302 |
| **Output Noise** | High (thousands of 404s) | Low (only valid pages) |
| **SecLists Support** | Manual configuration | Automatic detection |
| **Result Quality** | Mixed (signal + noise) | High (signal only) |

---

## 🔍 Example Run

```bash
$ prune dir https://example.com

 ____  ____  _  _  __ _  ____ 
(  _ \(  _ \/ )( \(  ( \(  __)
 ) __/ )   /) \/ (/    / ) _) 
(__)  (__\_)\____/\_)__)(____)

Adaptive Discovery Engine
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Directory Discovery
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ℹ Target: https://example.com
✓ Found SecLists at: /home/user/SecLists
→ Using SecLists wordlist: common.txt
⚙ Loaded 4,713 words, prioritized by confidence
ℹ Starting scan with adaptive rate: 100 req/s

200 │ https://example.com/admin (5.2KB) ●●●
200 │ https://example.com/api (1.1KB) ●●●
200 │ https://example.com/login (3.4KB) ●●●
200 │ https://example.com/dashboard (8.9KB) ●●●
301 │ https://example.com/old-admin → /admin ●●○

⚙ Filtered out: 404, 403, 429, 500, 502, 504
⚙ Only showing valid pages (200 OK) and redirects

→ ████████████████████████████████ 100% │ 94.7 req/s │ 5 discoveries

Intelligence Summary
────────────────────────────────────────────────────────────
  Showing only: 200 OK, 301/302 redirects
  Excluded codes: [404, 403, 429, 500, 502, 504, 405, 501]
  Wildcard patterns: 1
  Generated mutations: 18
  Overall confidence: 84.2%

✓ Directory discovery complete!
```

---

## 🐛 Troubleshooting

### "SecLists not found"

**Solution:** Install SecLists in a common location:

```bash
git clone https://github.com/danielmiessler/SecLists.git ~/SecLists
```

### "No results found"

**Possible reasons:**
1. Target has no common paths (try other wordlists)
2. All paths return filtered status codes
3. Target is blocking/rate-limiting

**Try:**
```bash
# Include 403 to see forbidden resources
prune status exclude 404,429,500,502,504

# Use slower rate
prune rate slow
```

### "Want to see all status codes"

**Solution:** Clear the exclusion list:

```bash
# Edit config
nano ~/.prune/config.json

# Set: "excluded_status_codes": []
```

---

## 📝 Technical Details

### Code Changes

1. **`src/wordlist.rs`**
   - Added `find_seclists()` method to detect SecLists
   - Modified `load_directory_wordlist()` to prefer SecLists
   - Modified `load_subdomain_wordlist()` to prefer SecLists
   - Added fallback to built-in wordlists

2. **`src/intelligence.rs`**
   - Updated default excluded codes: `[404, 403, 429, 500, 502, 504, 405, 501]`
   - Modified `is_successful_status()` to only consider 200-299 and 301/302
   - Improved learning to focus on valid responses

3. **`src/session.rs`**
   - Updated `ScanConfig::default()` with new excluded codes
   - Added comments explaining the filtering strategy

4. **`README.md`**
   - Added SecLists installation instructions
   - Updated features table with filtering info
   - Updated example output
   - Added data storage section for SecLists
   - Updated configuration examples

---

## 🎉 Benefits Summary

✅ **Cleaner Results** - No more noise from error pages  
✅ **Better Coverage** - Industry-standard SecLists wordlists  
✅ **Faster Analysis** - Focus on what matters  
✅ **Auto-Detection** - No manual configuration needed  
✅ **Smart Defaults** - Works perfectly out of the box  
✅ **Still Flexible** - Can customize if needed  

---

**Ready to test? Just run:** `prune dir https://example.com`

The tool will automatically find SecLists and show you only valid pages! 🚀
