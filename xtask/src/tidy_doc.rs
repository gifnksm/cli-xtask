use std::{
    any::Any,
    io::{BufWriter, Write},
    iter,
    process::Command,
};

use cli_xtask::{
    camino::Utf8Path, cargo_metadata::Metadata, clap, config::Config, eyre, process::CommandExt,
    tracing, workspace, Result, Run,
};

use crate::util;

/// `tidy-doc` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct TidyDoc {}

impl TidyDoc {
    /// Runs the `tidy-doc` subcommand.
    #[tracing::instrument(name = "tidy-doc", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {} = self;

        let workspace = workspace::current();
        let doc_dir = workspace.workspace_root.join("doc");
        emit_doc(workspace, &doc_dir)?;

        Ok(())
    }
}

impl Run for TidyDoc {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
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

pub fn emit_doc(workspace: &Metadata, doc_dir: &Utf8Path) -> Result<()> {
    cli_xtask::fs::create_or_cleanup_dir(doc_dir)?;

    let readme_path = doc_dir.join("README.md");
    let readme = cli_xtask::fs::create_file(&readme_path)?;
    let mut readme = BufWriter::new(readme);

    writeln!(&mut readme, "# `cargo-xtask` command reference")?;
    writeln!(&mut readme)?;

    let help = Command::new("cargo")
        .args(["run", "--all-features", "--", "--help"])
        .workspace_stdout(workspace)?;

    let subcommands = util::subcommands_from_help(&help);
    for subcommand in iter::once(None).chain(subcommands.into_iter().map(Some)) {
        let fullname = emit_markdown(workspace, doc_dir, subcommand)?;
        writeln!(readme, "* [`{fullname}(1)`]({fullname}.md)")?;
    }

    readme.into_inner()?.sync_all()?;

    Ok(())
}

fn emit_markdown(
    workspace: &Metadata,
    doc_dir: &Utf8Path,
    subcommand: Option<&str>,
) -> eyre::Result<String> {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--all-features", "--", "help"]);
    if let Some(subcommand) = subcommand {
        cmd.arg(subcommand);
    }

    let help = cmd.workspace_stdout(workspace)?;

    let fullname = subcommand
        .map(|subcommand| format!("cargo-xtask-{subcommand}"))
        .unwrap_or_else(|| "cargo-xtask".into());

    let output_path = doc_dir.join(format!("{fullname}.md"));
    let file = cli_xtask::fs::create_file(&output_path)?;
    let mut file = BufWriter::new(file);

    writeln!(
        &mut file,
        "\
        # `{fullname}(1)`\n\
        \n\
        ```test\n\
        {help}\
        ```\
        "
    )?;
    file.into_inner()?.sync_all()?;

    Ok(fullname)
}
