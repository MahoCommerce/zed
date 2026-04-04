# Maho for Zed

[Maho Intelligence](https://mahocommerce.com) LSP integration for the [Zed](https://zed.dev) editor.

Provides code completion, hover information, go-to-definition, and diagnostics for Maho's class alias system in PHP files.

## Prerequisites

- [Maho](https://mahocommerce.com) 26.5 or later
- PHP available on your PATH (or configured via settings)

## Setup

1. Install the extension from Zed's Extensions panel
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

## Supported Calls

### Models
- `Mage::getModel('catalog/product')`
- `Mage::getSingleton('catalog/product')`

### Helpers
- `Mage::helper('catalog')`

### Blocks
- `$layout->createBlock('catalog/product_list')`
- `$layout->getBlockSingleton('catalog/product_list')`

### Resource Models
- `Mage::getResourceModel('catalog/product')`
- `Mage::getResourceSingleton('catalog/product')`

### Config Paths
- `Mage::getStoreConfig('web/secure/base_url')`
- `Mage::getStoreConfigFlag('web/secure/use_in_frontend')`

### Events
- `Mage::dispatchEvent('catalog_product_save_after')`

## Features

- **Completion** — suggests aliases and paths as you type inside any of the supported calls
- **Hover** — shows the resolved PHP class name for a class alias
- **Go-to-definition** — jumps to the PHP class file for a class alias
- **Diagnostics** — reports unresolved class aliases

## License

MIT
