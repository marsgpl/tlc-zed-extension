use zed_extension_api as zed;

struct TLCExtension;

impl zed::Extension for TLCExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> &zed::Result<zed::Command> {
        let path = worktree.which("tlc").ok_or_else(|| {
            "please install tlc manually and make sure it is available in the PATH variable".to_string()
        })?;

        Ok(zed::Command {
            command: path,
            args: vec!["--server".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TLCExtension);
