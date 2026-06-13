# Maho for Zed

[![Get from Marketplace](https://img.shields.io/badge/Get_from-Marketplace-000000?style=for-the-badge&logo=zedindustries&logoColor=white)](https://zed.dev/extensions/maho-lsp)

[Maho Intelligence](https://mahocommerce.com) integration for the [Zed](https://zed.dev) editor.

Provides two things:

- **Editor intelligence (LSP)** — code completion, hover information, go-to-definition, and diagnostics for Maho's class alias system across PHP and XML files.
- **AI agent intelligence (MCP)** — exposes Maho's runtime configuration to Zed's AI agent through a [Model Context Protocol](https://modelcontextprotocol.io) server, so the agent can resolve aliases, inspect rewrites, read the merged config, and more instead of guessing.

## Prerequisites

- [Maho](https://mahocommerce.com) 26.5 or later
- PHP available on your PATH (or configured via settings)

## Setup

1. Install the extension:
   - **Zed Extensions** (recommended): Install [Maho](https://zed.dev/extensions/maho-lsp) from the Zed marketplace, or run `zed: extensions` from the command palette and search for "Maho"
   - **Manual install**: Clone this repository, then run `zed: install dev extension` from the command palette and select the cloned directory
2. Create a `.zed/settings.json` in your Maho project root:

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "maho-intelligence-lsp", "!phpactor", "..."]
    }
  }
}
```

This configures:
- **Intelephense** for PHP class/method completion (e.g. `Mage::getModel`)
- **Maho Intelligence LSP** for class alias completion (e.g. `'catalog/product'`)
- Disables **Phpactor**, which doesn't support Maho's non-namespaced classes well

> **Note:** Intelephense requires [Node.js](https://nodejs.org) installed on your system. Zed will download Intelephense automatically on first use.

## Editor (LSP) configuration

By default, the extension auto-detects `php` on your PATH and the `maho` CLI in the project root. You can override this in your Zed settings:

### Custom PHP path

```json
{
  "lsp": {
    "maho-intelligence-lsp": {
      "binary": {
        "path": "/usr/local/bin/php8.3",
        "arguments": ["./maho", "dev:lsp:start"]
      }
    }
  }
}
```

### Docker

```json
{
  "lsp": {
    "maho-intelligence-lsp": {
      "binary": {
        "path": "docker",
        "arguments": ["compose", "exec", "-T", "php", "php", "./maho", "dev:lsp:start"]
      }
    }
  }
}
```

## AI agent integration (MCP)

The extension also registers a [Model Context Protocol](https://modelcontextprotocol.io) server (`maho-intelligence-mcp`) that gives Zed's AI agent direct access to your project's Maho configuration, so it can understand how the application is wired instead of inferring from source alone.

It runs `php maho dev:mcp:start` from the project root and exposes tools for:

- Resolving and listing class aliases (model, block, helper, resource model)
- Listing class rewrites with conflict detection
- Reading the merged XML configuration for any path
- Listing modules, events/observers, cron jobs, and routes
- Inspecting EAV entity types and attributes
- Reading database table schemas, ACL resources, the admin menu, and the store hierarchy
- Resolving template fallback chains

### Enabling it

Zed disables every MCP server by default and requires you to opt in (running an MCP server feeds project data to the AI agent). Enable it in one of two ways:

- **Per machine:** open the **Agent Panel**, go to its settings, find **Maho** under *Model Context Protocol (MCP) Servers*, and flip the toggle on.
- **Per project (recommended for teams):** commit the following to your project's `.zed/settings.json` so it's on for everyone who opens the project:

```json
{
  "context_servers": {
    "maho-intelligence-mcp": {
      "source": "extension",
      "enabled": true,
      "settings": {}
    }
  }
}
```

Once enabled, Zed launches the server and the status dot turns green when it connects.

### Configuration

By default the server is launched as `php maho dev:mcp:start`, relying on `php` being on Zed's `PATH` and the `maho` CLI being in the project root.

> **Note:** Zed ignores the `command` field for extension-provided servers — it always invokes the extension's own launch command. So you can't override the full command the way you can for the LSP. Use the `php_path` setting below instead, or define a separate custom server (see Docker).

If `php` isn't found (for example, when Zed is launched from the macOS Dock with a minimal `PATH`), set an absolute path via the server's `settings`:

```json
{
  "context_servers": {
    "maho-intelligence-mcp": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "php_path": "/opt/homebrew/bin/php"
      }
    }
  }
}
```

### Docker

Because the extension server can only customize the PHP binary (not the whole command), run it through Docker as a separate **custom** server instead:

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

## Features

The editor (LSP) features below work across both **PHP** and **XML** files.

### Completion

Suggests aliases and paths as you type (triggered by `'` and `"` characters).

**PHP contexts:**

| Call | Example |
|------|---------|
| Model aliases | `Mage::getModel('catalog/product')` |
| Model aliases | `Mage::getSingleton('catalog/product')` |
| Resource model aliases | `Mage::getResourceModel('catalog/product')` |
| Resource model aliases | `Mage::getResourceSingleton('catalog/product')` |
| Helper aliases | `Mage::helper('catalog')` |
| Block aliases | `$layout->createBlock('catalog/product_list')` |
| Block aliases | `$layout->getBlockSingleton('catalog/product_list')` |
| Config paths | `Mage::getStoreConfig('web/secure/base_url')` |
| Config paths | `Mage::getStoreConfigFlag('web/secure/use_in_frontend')` |
| Event names | `Mage::dispatchEvent('catalog_product_save_after')` |

**XML contexts:**

Completion is context-aware based on XML tag and ancestry:

- `<class>` tags — model alias or FQCN depending on parent path (observers, rewrites, class prefixes, etc.)
- `<source_model>`, `<backend_model>` — model aliases
- `<frontend_model>`, `<render>`, `<renderer>` — block aliases
- `<block type="...">` attribute — block aliases
- `<template>` tag and `template` attribute — template paths
- `<model>` inside cron jobs — model alias with method callback (e.g. `catalog/product_action::run`)
- `ifconfig` attribute — config paths
- `handle` attribute — layout handles

### Hover

Shows context-sensitive documentation at cursor position.

- **Class aliases** (model, helper, block, resource model) — resolved PHP class name, file path, and rewrite info if applicable
- **Event names** — all registered observers grouped by area (frontend, admin), with class, method, and observer name
- **Config paths** — field label, section/group hierarchy, type, and default value
- **Fully qualified class names** in XML — class name and file location
- **XML methods** — method name, parent class, and method signature extracted from source
- **Cron callbacks** — model alias, method, class details, and method signature
- **Template paths** — resolved file location in theme directories
- **Layout handles** — handle name, defining file, and block count

### Go-to-definition

Jumps to the source file for:

- Class aliases (model, helper, block, resource model) → class file
- Fully qualified class names in XML → class file
- XML methods → method line in the class file
- Cron callbacks → class file or method line
- Template paths → template file in the design directory

### Diagnostics

Reports unresolved aliases as warnings (source: `maho-intelligence`). Diagnostics run automatically with a 0.3s debounce on document changes.

**PHP** — detects unresolved aliases in all `Mage::getModel()`, `Mage::getSingleton()`, `Mage::helper()`, `Mage::getResourceModel()`, `Mage::getResourceSingleton()`, `->createBlock()`, and `->getBlockSingleton()` calls.

**XML** — detects unresolved aliases in `<class>`, `<source_model>`, `<backend_model>`, `<frontend_model>`, `<render>`, `<renderer>`, `<block type="...">`, and `<model>` (cron callback) contexts.

## License

MIT
