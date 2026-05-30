# First Read Context Menu Tools

This repo builds one tiny portable Rust app (`read-first.exe`) and adds an Explorer context menu group:

- `Read first...` (parent menu)
  - `Line` - reads and shows only the first line.
  - `Megabyte` - reads and shows up to the first 1 MB.

Both entries call the same EXE with different mode flags. Output is shown in a read-only text box (select/copy supported, editing disabled).

## Binary placeholder behavior

The renderer displays bytes as:

- Printable ASCII (`0x20` to `0x7E`) as characters.
- New lines and tabs as normal whitespace.
- Other bytes (including `0x00`) as placeholders like `<00>`, `<1F>`, `<FF>`.

## Install

Download the release zip, extract it, then run from PowerShell in the extracted folder:

```powershell
.\install.ps1
```

This will:

1. Copy the bundled `read-first.exe` to `%LOCALAPPDATA%\FirstReadMenu`.
2. Register context menu entries under `HKCU:\Software\Classes\*\shell`.

No administrator rights are required.
The installer does not require the Rust toolchain.

## Build

For local development:

```powershell
cargo build --release
```

## Uninstall

```powershell
.\uninstall.ps1
```

This removes:

- The two context menu entries.
- `%LOCALAPPDATA%\FirstReadMenu\read-first.exe` and optional sidecar files (if present).
