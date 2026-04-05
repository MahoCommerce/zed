# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Maho for Zed — a Zed editor extension that integrates the Maho Intelligence LSP server for the Maho ecommerce platform. It provides PHP class alias completion, hover, go-to-definition, and diagnostics for Maho's class alias system (e.g. `Mage::getModel('catalog/product')`).

## Build & Development

This is a Zed extension compiled to WebAssembly. The sole dependency is `zed_extension_api`.

```bash
# Build the extension (produces extension.wasm via cdylib target)
cargo build --release --target wasm32-wasip1

# Check without building
cargo check --target wasm32-wasip1
```

The wasm32-wasip1 target must be installed: `rustup target add wasm32-wasip1`.

There are no tests or linting configured in this project.

## Architecture

Single-file extension (`src/lib.rs`) implementing `zed::Extension` trait:

- **`MahoExtension`** — registers with Zed via `zed::register_extension!` and implements `language_server_command()` which:
  1. Checks for user-configured binary override in Zed's LSP settings
  2. Falls back to auto-detection: finds `maho` CLI in worktree root + `php` on PATH
  3. Returns a command that runs `php ./maho dev:lsp:start`

The extension itself does not contain the LSP server — it delegates to the `maho` CLI (part of the Maho ecommerce framework, v26.5+) which runs the actual LSP.
