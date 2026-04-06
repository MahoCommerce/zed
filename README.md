# Maho for Zed

[Maho Intelligence](https://mahocommerce.com) LSP integration for the [Zed](https://zed.dev) editor.

Provides code completion, hover information, go-to-definition, and diagnostics for Maho's class alias system across PHP and XML files.

## Prerequisites

- [Maho](https://mahocommerce.com) 26.5 or later
- PHP available on your PATH (or configured via settings)

## Setup

1. Install the extension:
   - **Zed Extensions**: Marketplace listing is work in progress
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

## Configuration

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

## Features

All features work across both **PHP** and **XML** files.

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
