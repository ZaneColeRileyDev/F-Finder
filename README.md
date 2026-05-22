# F-Finder

> ⚠️ **Windows Only** — F-Finder has only been tested on Windows. Linux/macOS support is not guaranteed.

A fast, async command-line file search tool written in Rust. Search your drives for files by name, extension, or both — and get the results as a structured JSON file.

---

## Features

- Search files by **name** (partial match supported)
- Search files by **extension**
- Search by **both** name and extension simultaneously
- Scans a **specific drive** or **all drives** at once
- Outputs results as a **timestamped JSON file** containing matched file paths
- Async search using **Tokio** for concurrent multi-drive scanning

---

## Installation

Clone the repository and build with Cargo:

```bash
git clone <your-repo-url>
cd f-finder
cargo build --release
```

The compiled binary will be at `target/release/f-finder.exe`.

---

## Usage

```
f-finder --drive-letter <LETTER> [OPTIONS]
```

### Arguments

| Argument | Required | Description |
|---|---|---|
| `--drive-letter <LETTER>` | Yes (unless `--all-drives`) | The drive letter to search (e.g. `C`, `E`) |
| `--term <TERM>` | No* | Search term — matches any file whose name **contains** this value |
| `--ext <EXT>` | No* | File extension to filter by (e.g. `odt`, `txt`, `rs`) |
| `--all-drives` | No | Scan all available drives instead of a single one |

> **\*Note:** At least one of `--term` or `--ext` must be provided.

---

## Examples

Search for all `.odt` files on drive `E`:
```bash
f-finder --drive-letter E --ext odt
```

Search for files with "roadmap" in the name on drive `C`:
```bash
f-finder --drive-letter C --term roadmap
```

Search for `.rs` files with "search" in the name across all drives:
```bash
f-finder --drive-letter C --all-drives --term search --ext rs
```

---

## Output

Results are written to a JSON file inside a `F-Finder-Data/json/` folder created in the current working directory. The file is named using the drive letter and a Unix timestamp:

```
F-Finder-Data/
└── json/
    └── E_1779481278.json
```

Each entry in the JSON file contains:

```json
[
  {
    "path": "E:/Documents/MyFile.odt",
    "id": 0,
    "timestamp": "2026-05-22T20:13:02.631590600Z"
  }
]
```

| Field | Description |
|---|---|
| `path` | Full absolute path to the matched file |
| `id` | Zero-based index of the result within this search run |
| `timestamp` | UTC time the match was recorded during the search |

---

## Project Structure

```
src/
├── main.rs           # CLI argument parsing, search orchestration, output
└── algorithim/
    ├── mod.rs        # Module re-exports
    ├── models.rs     # OutputPath and OutputJson data structures
    └── search.rs     # File walking and search logic, drive detection
```

---

## Dependencies

- [`clap`](https://crates.io/crates/clap) — CLI argument parsing
- [`tokio`](https://crates.io/crates/tokio) — Async runtime
- [`walkdir`](https://crates.io/crates/walkdir) — Recursive directory traversal
- [`sysinfo`](https://crates.io/crates/sysinfo) — Drive/disk enumeration
- [`whoami`](https://crates.io/crates/whoami) — Current username (for `C:/Users/<name>` resolution)
- [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) — JSON serialization
- [`humantime-serde`](https://crates.io/crates/humantime-serde) — Human-readable timestamps in JSON

---

## Notes

- When searching drive `C`, the tool automatically scopes the search to `C:/Users/<your-username>` to avoid scanning system directories unnecessarily.
- Running `--ext` alone without `--term` will scan for **every file** with that extension across the entire drive — this can be slow on large drives. It is recommended to pair `--ext` with `--term` when possible.
- The `--all-drives` flag will spawn a concurrent async task per drive, so results are written as separate JSON files, one per drive.
