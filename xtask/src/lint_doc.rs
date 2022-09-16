use std::any::Any;

use tempfile::TempDir;

use cli_xtask::{
    camino::Utf8Path, clap, config::Config, eyre::eyre, tracing, workspace, Error, Result, Run,
    SubcommandRun,
};

/// `lint-doc` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct LintDoc {}

impl LintDoc {
    /// Runs the `lint-doc` subcommand.
    #[tracing::instrument(name = "lint-doc", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {} = self;

        let workspace = workspace::current();
        let doc_dir = workspace.workspace_root.join("doc");

        let reference_dir = TempDir::new()?;
        super::tidy_doc::emit_doc(workspace, <&Utf8Path>::try_from(reference_dir.path())?)?;

        if dir_diff::is_different(&doc_dir, &reference_dir).map_err(|e| -> Error {
            match e {
                dir_diff::Error::Io(e) => e.into(),
                dir_diff::Error::StripPrefix(e) => e.into(),
                dir_diff::Error::WalkDir(e) => e.into(),
            }
        })? {
            return Err(eyre!("doc directory is not up to date"));
        }

        Ok(())
    }
}

impl Run for LintDoc {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }

    fn to_subcommands(&self) -> Option<SubcommandRun> {
        None
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
