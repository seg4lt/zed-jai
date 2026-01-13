use zed::{LanguageServerId, Worktree};
use zed_extension_api::{self as zed, Result, serde_json, settings::LspSettings};

struct JaiExtension;
struct JailsBinary {
    path: String,
    args: Option<Vec<String>>,
}

impl JaiExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<JailsBinary> {
        let language_server = language_server_id.as_ref();
        if let Ok(lsp_settings) = LspSettings::for_worktree(language_server, worktree)
            && let Some(binary) = lsp_settings.binary
            && let Some(path) = binary.path
        {
            return Ok(JailsBinary {
                path,
                args: binary.arguments,
            });
        }

        if let Some(path) = worktree.which(language_server) {
            return Ok(JailsBinary { path, args: None });
        }

        Err("Unable to locate jails binary".to_string())
    }
}

impl zed::Extension for JaiExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let jails_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: jails_binary.path,
            args: jails_binary.args.unwrap_or_default(),
            env: worktree.shell_env(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(JaiExtension);
