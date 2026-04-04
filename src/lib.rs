use std::{env, fs};
use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct MahoExtension;

impl zed::Extension for MahoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Check for user-configured binary override
        let lsp_settings =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        if let Some(binary) = lsp_settings.binary {
            if let Some(path) = binary.path {
                return Ok(zed::Command {
                    command: path,
                    args: binary.arguments.unwrap_or_default(),
                    env: binary
                        .env
                        .unwrap_or_default()
                        .into_iter()
                        .collect(),
                });
            }
        }

        // Auto-detect: find maho CLI in worktree root
        let worktree_root = env::current_dir()
            .map_err(|e| format!("could not determine working directory: {e}"))?;

        let maho_path = worktree_root.join("maho");

        if !fs::metadata(&maho_path).is_ok_and(|m| m.is_file()) {
            return Err(
                "This does not appear to be a Maho project (no `maho` file found in the project root)."
                    .to_string(),
            );
        }

        let php_path = worktree.which("php").ok_or_else(|| {
            "Could not find `php` on PATH. PHP is required to run the Maho Intelligence LSP server."
                .to_string()
        })?;

        Ok(zed::Command {
            command: php_path,
            args: vec![
                maho_path.to_string_lossy().to_string(),
                "dev:lsp:start".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(MahoExtension);
