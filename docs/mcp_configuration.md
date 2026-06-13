# Maho Intelligence MCP

Provided by the **Maho for Zed** extension. This server runs
`php maho dev:mcp:start` from your project's root directory and gives Zed's AI
agent direct access to your Maho configuration (class aliases, rewrites, merged
config, modules, events, EAV attributes, DB schema, and more).

## Requirements

- A [Maho](https://mahocommerce.com) project — the `maho` CLI must exist in the
  worktree root.
- PHP must be runnable by Zed.

## Settings

- **`php_path`** — Path to the PHP binary. Defaults to `php` from `$PATH`.

If the server times out or fails to start, Zed most likely could not find PHP
on its `$PATH`. This commonly happens when Zed is launched from the macOS Dock,
which provides a minimal `$PATH` that excludes Homebrew. Either relaunch Zed
from a terminal (`zed .`), or set `php_path` to an absolute path.

### macOS (Homebrew)

Apple Silicon — Homebrew installs to `/opt/homebrew`:

```json
{
  "php_path": "/opt/homebrew/bin/php"
}
```

Intel Macs — Homebrew installs to `/usr/local`:

```json
{
  "php_path": "/usr/local/bin/php"
}
```

Run `which php` in your terminal to confirm the exact path.

## Docker

To run the server inside a container, define it as a custom (non-extension)
server instead, since a custom command requires a different executable:

```json
{
  "context_servers": {
    "maho-mcp-docker": {
      "source": "custom",
      "command": "docker",
      "args": ["compose", "exec", "-T", "php", "php", "maho", "dev:mcp:start"]
    }
  }
}
```
