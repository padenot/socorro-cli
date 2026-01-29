# socorro-cli

A Rust CLI tool for querying Mozilla's Socorro crash reporting system, optimized for LLM coding agents.

Written by Claude Code *NOT YET REVIEWED THOROUGHLY***.

## Installation

```bash
cargo install --path .
```

## Usage

### Crash Command

Fetch details about a specific crash by ID or URL:

```bash
# Using crash ID
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120

# Using full Socorro URL (copy-paste from browser)
socorro-cli crash https://crash-stats.mozilla.org/report/index/247653e8-7a18-4836-97d1-42a720260120

# Get full crash data without omissions
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --full

# Limit stack trace depth
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --depth 5

# Different output formats
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --format markdown
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --format json
```

### Search Command

Search and aggregate crashes with filters:

```bash
# Basic search
socorro-cli search --signature "AudioDecoderInputTrack" --product Fenix

# Search with filters
socorro-cli search --product Firefox --platform Windows --days 30 --limit 20

# Aggregate by fields
socorro-cli search --product Firefox --days 7 --facet platform --facet version

# Sort results
socorro-cli search --product Firefox --days 1 --sort -date --limit 10
```

## Output Formats

### Compact (default)
Token-optimized plain text format designed for LLMs:
```
CRASH 247653e8-7a18-4836-97d1-42a720260120
sig: mozilla::AudioDecoderInputTrack::EnsureTimeStretcher
reason: SIGSEGV / SEGV_MAPERR @ 0x0 (null ptr)
product: Fenix 147.0.1 (Android 36, SM-S918B)

stack[GraphRunner]:
  #0 EnsureTimeStretcher @ AudioDecoderInputTrack.cpp:624
  #1 AppendTimeStretchedDataToSegment @ AudioDecoderInputTrack.cpp:423
```

### JSON
Full structured data for programmatic processing.

### Markdown
Formatted output for documentation and chat interfaces.

## Options

### Global Options
- `--format <FORMAT>`: Output format (compact, json, markdown) [default: compact]

### Crash Options
- `--depth <N>`: Stack trace depth [default: 10]
- `--full`: Output complete crash data without omissions (forces JSON format)
- `--all-threads`: Show stacks from all threads (useful for diagnosing deadlocks)

### Search Options
- `--signature <SIG>`: Filter by crash signature (supports wildcards)
- `--product <PROD>`: Filter by product [default: Firefox]
- `--version <VER>`: Filter by version
- `--platform <PLAT>`: Filter by platform (Windows, Linux, Mac, Android)
- `--days <N>`: Search crashes from last N days [default: 7]
- `--limit <N>`: Maximum results to return [default: 10]
- `--facet <FIELD>`: Aggregate by field (can be repeated)
- `--sort <FIELD>`: Sort field [default: -date]

## Examples

### Basic Crash Investigation

```bash
# Quick crash lookup (compact format, default)
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120

# Output:
# CRASH 247653e8-7a18-4836-97d1-42a720260120
# sig: mozilla::AudioDecoderInputTrack::EnsureTimeStretcher
# reason: SIGSEGV / SEGV_MAPERR @ 0x0000000000000000
# moz_reason: MOZ_RELEASE_ASSERT(mTimeStretcher->Init())
# product: Fenix 147.0.1 (Android 36, SM-S918B 36 (REL))
#
# stack[GraphRunner]:
#   #0 mozilla::AudioDecoderInputTrack::EnsureTimeStretcher() @ AudioDecoderInputTrack.cpp:624
#   #1 mozilla::AudioDecoderInputTrack::AppendTimeStretchedDataToSegment(...) @ AudioDecoderInputTrack.cpp:423
#   ...

# Copy-paste URL directly from browser
socorro-cli crash https://crash-stats.mozilla.org/report/index/247653e8-7a18-4836-97d1-42a720260120

# Show only top 3 frames for quick overview
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --depth 3
```

### Deadlock and Multi-threading Issues

```bash
# Show all thread stacks (useful for diagnosing deadlocks, race conditions)
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --all-threads --depth 5

# Output shows all threads with the crashing thread marked:
# stack[thread 0:la.firefox:tab7]:
#   #0 ???
#   ...
#
# stack[thread 49:GraphRunner [CRASHING]]:
#   #0 mozilla::AudioDecoderInputTrack::EnsureTimeStretcher() @ ...
#   #1 mozilla::AudioDecoderInputTrack::AppendTimeStretchedDataToSegment(...) @ ...
#   ...
#
# stack[thread 50:MediaDecoderSta]:
#   #0 mozilla::SharedBuffer::Create(...) @ ...
#   ...

# All threads with minimal depth for overview
socorro-cli crash <crash-id> --all-threads --depth 2
```

### Output Formats

```bash
# Markdown format for documentation or bug reports
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --format markdown

# JSON for programmatic processing
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --format json | jq '.signature'

# Full JSON dump without any omissions (includes all metadata)
socorro-cli crash 247653e8-7a18-4836-97d1-42a720260120 --full
```

### Search and Aggregation

```bash
# Find recent crashes with specific signature
socorro-cli search --signature "AudioDecoderInputTrack" --product Fenix --days 30

# Output:
# FOUND 803 crashes
#
# 5403b258 | Fenix 147.0.1 | Unknown | mozilla::AudioDecoderInputTrack::EnsureTimeStretcher
# 5b7622f7 | Fenix 147.0.1 | Unknown | mozilla::AudioDecoderInputTrack::EnsureTimeStretcher
# ...

# Aggregate crashes by platform and version
socorro-cli search --product Firefox --days 7 --facet platform --facet version --limit 5

# Output:
# FOUND 69146 crashes
#
# 6df5bc35 | Firefox 143.0 | Unknown | OOM | small
# ...
#
# AGGREGATIONS:
#
# version:
#   146.0.1 (407)
#   147.0.1 (179)
#   ...
#
# platform:
#   Windows (45000)
#   Linux (12000)
#   ...

# Find crashes on specific platform and version
socorro-cli search --product Firefox --platform Windows --version 147.0.1 --days 14

# Top crashes by signature
socorro-cli search --product Firefox --days 7 --facet signature --limit 20

# Recent Android crashes
socorro-cli search --product Fenix --platform Android --days 3 --limit 20
```

### Common Workflows

```bash
# Investigate a crash from triage
socorro-cli crash <crash-id> --depth 15 --format markdown > crash-analysis.md

# Quick signature search to find related crashes
socorro-cli search --signature "MyFunction" --days 30 --limit 10

# Check if a crash affects multiple versions
socorro-cli search --signature "MyFunction" --facet version --days 30

# Deadlock investigation workflow
# 1. Get crash with all threads
socorro-cli crash <deadlock-crash-id> --all-threads --depth 10 > deadlock-stacks.txt
# 2. Review all thread stacks to identify lock holders and waiters

# Check crash distribution across platforms
socorro-cli search --signature "MyFunction" --facet platform --days 7
```

## License

MPL 2.0
