# Prune GUI Guide

## 🎨 Graphical User Interface

Prune now includes a **beautiful graphical interface** for users who prefer visual tools over command-line interfaces.

---

## 🚀 Launching the GUI

### From Command Line

```bash
prune gui
```

That's it! The graphical interface will open automatically.

### From Binary

```bash
cd Prune
./target/release/prune gui
```

---

## 📋 GUI Features

### 1. **Target Configuration**
- **Target URL Input**: Enter your target URL or domain
- **Scan Mode Selection**:
  - 📁 Directory Discovery
  - 🌐 Subdomain Enumeration  
  - 🔄 Combined (Both)

### 2. **Scan Configuration**
All the CLI options are available in the GUI:

| Option | Description | Range |
|--------|-------------|-------|
| **Request Rate** | Control scan speed | 10-500 req/s |
| **Request Timeout** | Max wait per request | 5-60 seconds |
| **Passive Crawling** | Extract links from responses | On/Off |
| **Crawl Depth** | How deep to follow links | 1-10 levels |
| **Excluded Status Codes** | Filter unwanted responses | Checkboxes |

**Default Excluded Codes:**
- ✓ 404 - Not Found
- ✓ 403 - Forbidden
- ✓ 429 - Too Many Requests
- ✓ 500 - Internal Server Error
- ✓ 502 - Bad Gateway
- ✓ 504 - Gateway Timeout

### 3. **Wordlist Options**

#### Use SecLists (Recommended)
Leave "Use custom wordlist" unchecked to automatically use SecLists or built-in wordlists.

#### Use Custom Wordlist
1. Check "Use custom wordlist"
2. Click "📁 Browse..."
3. Navigate to your wordlist file
4. Select the `.txt` file
5. Start your scan

**Custom wordlist benefits:**
- Target-specific paths
- Industry-specific subdomains
- Client-provided wordlists
- Penetration test requirements

### 4. **Real-time Progress**
While scanning, the GUI displays:
- ⚙️ **Progress Bar** - Visual completion indicator
- 📊 **Request Rate** - Current requests per second
- 🎯 **Discoveries** - Valid findings count
- 📝 **Status Messages** - What Prune is doing

### 5. **Results View**
After scanning completes:

| Column | Description |
|--------|-------------|
| **Status** | HTTP status code (color-coded) |
| **URL** | Clickable link to finding |
| **Size** | Response size |
| **Confidence** | ●●● High, ●●○ Medium, ●○○ Low |

**Results Features:**
- 🔍 **Filter** - Search results by URL
- ✅ **Show only 200 OK** - Hide redirects and other codes
- 📥 **Export** - Save results to JSON or CSV

---

## 🎨 Color Scheme

The GUI uses Prune's signature colors:

| Color | Hex | Usage |
|-------|-----|-------|
| **Primary Blue** | #2596be | Titles, buttons, 200 OK responses |
| **Secondary Violet** | #5621d5 | Labels, metadata, 3xx responses |
| **Yellow** | #ffcc00 | 4xx client errors |
| **Red** | #ff6b6b | 5xx server errors |

---

## 📸 GUI Walkthrough

### Step 1: Launch

```bash
prune gui
```

### Step 2: Enter Target
```
Target URL: https://example.com
Scan Mode: ● Directory Discovery
```

### Step 3: Configure (Optional)
```
Request Rate: 100 req/s
Timeout: 10 seconds
☑ Enable passive crawling
```

### Step 4: Start Scan
Click **🚀 Start Scan** button

### Step 5: Monitor Progress
```
Progress: ████████████████░░░░ 65%
Rate: 89.3 req/s | Discoveries: 4
```

### Step 6: View Results
```
Status  URL                              Size      Confidence
200     https://example.com/admin        5.2KB     ●●●
200     https://example.com/api          1.1KB     ●●●
301     https://example.com/old → /new   0B        ●●○
```

### Step 7: Export (Optional)
Click **💾 Export Results** → Choose format (JSON/CSV) → Save

---

## 🔧 Advanced Features

### Custom Wordlist Workflow

1. **Prepare your wordlist** (`my-wordlist.txt`):
```
admin
dashboard
api/v1
api/v2
config
```

2. **In GUI**:
   - ☑ Use custom wordlist
   - Click 📁 Browse...
   - Select `my-wordlist.txt`
   - Start scan

3. **Prune will**:
   - Use your custom paths
   - Apply intelligence filtering
   - Generate smart mutations

### Combining Features

**Maximum coverage scan:**
```
Target: https://target.com
Mode: Combined (Both)
Rate: 100 req/s
☑ Enable passive crawling
Depth: 3
☑ Use custom wordlist: client-specific.txt
```

**Quiet stealth scan:**
```
Target: https://target.com
Mode: Directory Discovery
Rate: 25 req/s
☐ Disable passive crawling
Excluded: [all except 200]
```

---

## 💾 Exporting Results

### JSON Format
Perfect for automation and parsing:

```json
[
  {
    "url": "https://example.com/admin",
    "status_code": 200,
    "size": 5324,
    "confidence": 0.95,
    "discovered_at": "2026-01-12T15:30:00Z"
  }
]
```

### CSV Format
Perfect for spreadsheets and reports:

```csv
Status,URL,Size,Confidence,Discovered At
200,https://example.com/admin,5324,0.95,2026-01-12T15:30:00Z
200,https://example.com/api,1125,0.89,2026-01-12T15:30:05Z
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+C` | Stop scan |
| `Ctrl+F` | Focus filter box |
| `Ctrl+E` | Export results |
| `Escape` | Close dialogs |

---

## 🐛 Troubleshooting

### GUI Doesn't Launch

**Issue**: `prune gui` shows error

**Solutions**:
1. Ensure you built with `cargo build --release`
2. Check GUI dependencies are installed:
   ```bash
   # Linux
   sudo apt install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
   
   # Fedora
   sudo dnf install gtk3-devel
   ```

### Scan Gets Stuck

**Issue**: Progress bar doesn't move

**Solutions**:
1. Click "⏹ Stop Scan"
2. Check target URL is correct
3. Try lower request rate
4. Check internet connection

### No Results Found

**Issue**: Scan completes but shows 0 results

**Possible reasons**:
1. All responses were filtered (404, 403, etc.)
2. Target has no common paths
3. Target is blocking requests

**Solutions**:
1. Uncheck some excluded status codes
2. Try custom wordlist
3. Reduce request rate (avoid rate limiting)

### Custom Wordlist Not Working

**Issue**: Browse button doesn't respond

**Solutions**:
1. Ensure wordlist is `.txt` format
2. Check file permissions (must be readable)
3. Try absolute path

---

## 🆚 GUI vs CLI

| Feature | GUI | CLI |
|---------|-----|-----|
| **Ease of Use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Visual Feedback** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Automation** | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Remote Use** | ❌ | ✅ |
| **Resource Usage** | Higher | Lower |
| **Custom Wordlists** | Easy (Browse) | Easy (Path) |
| **Export Results** | Built-in | Manual |
| **Session Resume** | ❌ | ✅ |

**Use GUI when:**
- Learning Prune
- One-off scans
- Need visual feedback
- Want easy export

**Use CLI when:**
- Automating scans
- Remote servers
- Scripting
- Session resumption needed

---

## 🎓 Best Practices

### 1. Start Conservative
```
Rate: 50 req/s
Timeout: 10 seconds
Crawling: Off
```

### 2. Monitor Progress
Watch the progress bar and discoveries count. If rate drops significantly, target may be rate-limiting.

### 3. Use Filters
After scan completes:
1. Review all results first
2. Filter by specific paths
3. Toggle "Show only 200 OK" for clean view

### 4. Export Early
Export results immediately after scan completes to avoid losing data.

### 5. Custom Wordlists
For repeat targets, create custom wordlists based on previous discoveries.

---

## 🔒 Security Notes

### GUI Safety
- ⚠️ GUI stores no credentials
- ⚠️ Results are in-memory only until exported
- ⚠️ Closing GUI discards unsaved results
- ✅ All security features from CLI are active

### Permission Reminder
**Always obtain explicit permission before scanning:**
- Written authorization
- Clear scope definition
- Appropriate timing
- Legal compliance

The GUI makes scanning easier, but **legal and ethical obligations remain the same**.

---

## 💡 Tips & Tricks

### Tip 1: Progress Watching
The "Status" field updates as Prune works:
- "Initializing..." - Starting up
- "Loaded X words..." - Ready to scan
- "Testing paths..." - Actively scanning
- "Scan complete!" - Finished

### Tip 2: Quick Export
Right after scan completes, export immediately:
1. Click "💾 Export Results"
2. Choose format
3. Save with descriptive name: `target-scan-2026-01-12.json`

### Tip 3: Filter Patterns
Use filter to find specific patterns:
- Filter: "api" → See all API endpoints
- Filter: ".php" → See all PHP files
- Filter: "admin" → See all admin panels

### Tip 4: Confidence Indicators
- ●●● (High) = Definitely investigate
- ●●○ (Medium) = Worth checking
- ●○○ (Low) = Lower priority

### Tip 5: Rate Tuning
If discoveries drop to zero:
1. Lower rate (target may be rate-limiting)
2. Increase timeout (target may be slow)
3. Check excluded codes (may be filtering too much)

---

## 📞 Support

Having issues with the GUI?

1. **Check this guide** first
2. **Try CLI** version to compare
3. **Report bugs** at: https://github.com/declanmiddleton/Prune/issues
4. **Include**:
   - OS and version
   - Prune version
   - Steps to reproduce
   - Error messages

---

## 🚀 Future GUI Features

Planned enhancements:

- [ ] Dark/Light theme toggle
- [ ] Save/Load scan configurations
- [ ] Real-time results table updates
- [ ] Chart visualization of findings
- [ ] Multiple concurrent scans
- [ ] Session resumption in GUI
- [ ] Drag-and-drop wordlist upload
- [ ] Keyboard shortcuts customization

---

**The GUI makes Prune accessible to everyone, from beginners to experts!** 🌿

For CLI documentation, see [README.md](README.md) and [QUICKSTART.md](QUICKSTART.md).
