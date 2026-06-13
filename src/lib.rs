use zed_extension_api::{
    self as zed,
    settings::{ContextServerSettings, LspSettings},
    Result,
};

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
                    env: binary.env.unwrap_or_default().into_iter().collect(),
                });
            }
        }

        // Auto-detect: ensure this is a Maho project by checking for the
        // `maho` CLI file in the worktree root, using the Zed worktree API.
        if worktree.read_text_file("maho").is_err() {
            return Err(
                "This does not appear to be a Maho project (no `maho` file found in the project root)."
                    .to_string(),
            );
        }

        let maho_path = format!("{}/maho", worktree.root_path());

        let php_path = worktree.which("php").ok_or_else(|| {
            "Could not find `php` on PATH. PHP is required to run the Maho Intelligence LSP server."
                .to_string()
        })?;

        Ok(zed::Command {
            command: php_path,
            args: vec![maho_path, "dev:lsp:start".to_string()],
            env: Default::default(),
        })
    }

    fn context_server_command(
        &mut self,
        context_server_id: &zed::ContextServerId,
        project: &zed::Project,
    ) -> Result<zed::Command> {
        // Run `php maho dev:mcp:start`, which starts the Maho Intelligence MCP
        // server over stdio. Unlike `language_server_command`, the
        // context-server API only exposes worktree ids (no file access or
        // `which`), but Zed launches context servers with the project root as
        // the working directory — so `maho` resolves relative to it.
        //
        // Zed ignores the `command` field of context-server settings for
        // extension-provided servers, so the launch command lives here. The
        // `php` binary is taken from `$PATH`; users whose Zed can't find it
        // (e.g. launched from the macOS Dock with a minimal `$PATH`) can point
        // us at a specific binary via the `php_path` setting.
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;
        let php = settings
            .settings
            .as_ref()
            .and_then(|s| s.get("php_path"))
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("php")
            .to_string();

        Ok(zed::Command {
            command: php,
            args: vec!["maho".to_string(), "dev:mcp:start".to_string()],
            env: Default::default(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> Result<Option<zed::ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../docs/mcp_configuration.md").to_string();
        let default_settings = "{\n  \"php_path\": \"php\"\n}\n".to_string();
        let settings_schema = r#"{
  "type": "object",
  "properties": {
    "php_path": {
      "type": "string",
      "description": "Path to the PHP binary used to run `maho dev:mcp:start`. Defaults to `php` from $PATH. Set an absolute path (e.g. /opt/homebrew/bin/php) if Zed cannot find PHP — for example when launched from the macOS Dock."
    }
  }
}"#
        .to_string();

        Ok(Some(zed::ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(MahoExtension);
