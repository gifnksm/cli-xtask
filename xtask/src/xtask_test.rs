use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    process::Command,
};

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata,
};
use clap::Parser;

use cli_xtask::{config::Config, fs::ToRelative, process::CommandExt, workspace};

/// `xtask-test` subcommand arguments.
#[derive(Debug, Parser)]
pub struct XtaskTest {
    /// Collect coverage information using cargo-llvm-cov.
    #[clap(long)]
    cargo_llvm_cov: bool,
}

impl XtaskTest {
    /// Execute `xtask-test` subcommand workflow.
    #[tracing::instrument(name = "xtask-test", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
        let Self { cargo_llvm_cov } = self;

        for workspace in &workspace::all()[1..] {
            test_workspace(workspace, *cargo_llvm_cov)?;
        }

        Ok(())
    }
}

fn test_workspace(workspace: &Metadata, cargo_llvm_cov: bool) -> eyre::Result<()> {
    let mut envs = vec![];
    if cargo_llvm_cov {
        envs = cargo_llvm_cov_init(workspace)?;
    }
    let cargo = Cargo::new(workspace, envs);

    // build executable
    cargo.spawn(["build", "-p", "xtask"])?;

    // show help
    let help = cargo.stdout(["xtask", "--help"])?;
    assert_eq!(help.lines().next(), Some("cargo-xtask "));

    // extract subcommands from help message
    let subcommands = help
        .lines()
        .skip_while(|l| !l.starts_with("SUBCOMMANDS:"))
        .skip(1)
        .take_while(|l| l.starts_with("    "))
        .filter_map(|l| {
            let cmd = l.strip_prefix("    ")?.split_once(' ')?.0;
            (!cmd.is_empty()).then(|| cmd)
        })
        .collect::<Vec<_>>();
    tracing::info!("subcommands: {subcommands:?}");

    // no subcommands or help subcomand emit same help message with --help
    assert_eq!(cargo.stdout(["xtask"])?, help);
    assert_eq!(cargo.stdout(["xtask", "help"])?, help);

    for subcommand in &subcommands {
        let _span = tracing::info_span!("test", subcommand).entered();
        match *subcommand {
            "build" => cargo.spawn(["xtask", "build"])?,
            "clippy" => cargo.spawn(["xtask", "clippy"])?,
            "dist" => cargo.spawn(["xtask", "dist"])?,
            "dist-archive" => test_dist_archive(&cargo)?,
            "dist-build" => cargo.spawn(["xtask", "dist-build"])?,
            "dist-build-bin" => cargo.spawn(["xtask", "dist-build-bin"])?,
            "dist-build-completion" => cargo.spawn(["xtask", "dist-build-completion"])?,
            "dist-build-doc" => cargo.spawn(["xtask", "dist-build-doc"])?,
            "dist-build-license" => cargo.spawn(["xtask", "dist-build-license"])?,
            "dist-build-man" => cargo.spawn(["xtask", "dist-build-man"])?,
            "dist-build-readme" => cargo.spawn(["xtask", "dist-build-readme"])?,
            "dist-clean" => cargo.spawn(["xtask", "dist-clean"])?,
            "fmt" => cargo.spawn(["xtask", "fmt"])?,
            "help" => {}
            "lint" => cargo.spawn(["xtask", "lint"])?,
            "rdme" => cargo.spawn(["xtask", "rdme"])?,
            "test" => cargo.spawn(["xtask", "test"])?,
            "udeps" => cargo.spawn(["xtask", "udeps"])?,
            _ => panic!("unknown subcommand: {subcommand}"),
        }
    }

    if cargo_llvm_cov {
        cargo_llvm_cov_fini(&cargo)?;
    }
    Ok(())
}

fn test_dist_archive(cargo: &Cargo) -> eyre::Result<()> {
    // if working directory is empty, no artifacts and dist directory created
    cargo.cleanup()?;
    cargo.spawn(["xtask", "dist-archive"])?;
    assert!(!cargo.target_directory().join("dist").exists());

    let workdir = cargo.target_directory().join("xtask/dist/app-v0.1.0");
    let distdir = cargo.target_directory().join("dist");

    // archive created per archtecture
    cargo.cleanup()?;
    let noarch = workdir.join("noarch");
    fs::create_dir_all(&noarch)?;
    fs::create_dir_all(noarch.join("doc"))?;
    File::create(noarch.join("doc/text"))?.write_all(b"text")?;

    let arch1 = workdir.join("arch1");
    fs::create_dir_all(&arch1)?;
    File::create(arch1.join("binary"))?.write_all(b"arch1")?;

    let arch2 = workdir.join("arch2");
    fs::create_dir_all(&arch2)?;
    File::create(arch2.join("binary"))?.write_all(b"arch2")?;

    cargo.spawn(["xtask", "dist-archive"])?;
    assert!(distdir.join("app-v0.1.0-arch2.tar.gz").exists());
    assert!(distdir.join("app-v0.1.0-arch1.tar.gz").exists());
    assert!(!distdir.join("app-v0.1.0-noarch.tar.gz").exists());

    // noarch archive created if noarch directory exists and other directory not exists
    cargo.cleanup()?;
    let noarch = workdir.join("noarch");
    fs::create_dir_all(&noarch)?;
    fs::create_dir_all(noarch.join("doc"))?;
    File::create(noarch.join("doc/text"))?.write_all(b"text")?;

    cargo.spawn(["xtask", "dist-archive"])?;
    assert!(distdir.join("app-v0.1.0-noarch.tar.gz").exists());

    Ok(())
}

#[derive(Debug)]
struct Cargo<'a> {
    workspace: &'a Metadata,
    target_directory: Utf8PathBuf,
    envs: Vec<(String, String)>,
}

impl<'a> Cargo<'a> {
    fn new(workspace: &'a Metadata, envs: Vec<(String, String)>) -> Self {
        let target_env = envs.iter().find(|(k, _v)| k == "CARGO_TARGET_DIR");
        let target_directory = if let Some((_k, v)) = target_env {
            Utf8PathBuf::from(v)
        } else {
            workspace.target_directory.clone()
        };
        Self {
            workspace,
            target_directory,
            envs,
        }
    }

    fn target_directory(&self) -> &Utf8Path {
        &self.target_directory
    }

    fn build(&self) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.envs(self.envs.iter().map(|(k, v)| (k.as_str(), v.as_str())));
        cmd
    }

    fn spawn(&self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> eyre::Result<()> {
        self.build().args(args).workspace_spawn(self.workspace)
    }

    fn stdout_raw(
        &self,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> eyre::Result<Vec<u8>> {
        self.build().args(args).workspace_stdout(self.workspace)
    }

    fn stdout(&self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> eyre::Result<String> {
        let output = self.stdout_raw(args)?;
        Ok(String::from_utf8(output)?)
    }

    fn cleanup(&self) -> eyre::Result<()> {
        tracing::info!("cleaning {}", self.target_directory.to_relative());
        let dist_dir = self.target_directory.join("dist");
        if dist_dir.is_dir() {
            fs::remove_dir_all(&dist_dir)?;
        }
        let xtask_dir = self.target_directory.join("xtask");
        if xtask_dir.is_dir() {
            fs::remove_dir_all(&xtask_dir)?;
        }
        Ok(())
    }
}

fn cargo_llvm_cov_init(workspace: &Metadata) -> eyre::Result<Vec<(String, String)>> {
    let target_dir = workspace.target_directory.join("llvm-cov-target");

    // get environment variables to pass to the child process
    let output = Command::new("cargo")
        .args(["llvm-cov", "show-env"])
        .env("CARGO_TARGET_DIR", &target_dir)
        .workspace_stdout(workspace)?;

    let mut envs = vec![("CARGO_TARGET_DIR".to_string(), target_dir.into_string())];
    for line in String::from_utf8(output)?.lines() {
        let (k, v) = line
            .split_once('=')
            .ok_or_else(|| eyre::eyre!("invalid line: {}", line))?;
        let v = v.trim_matches('"');
        envs.push((k.to_string(), v.to_string()));
    }

    // remove remove artifacts that may affect the coverage results
    Command::new("cargo")
        .args(["llvm-cov", "clean", "--workspace"])
        .envs(envs.iter().map(|(k, v)| (k.as_str(), v.as_str())))
        .workspace_spawn(workspace)?;

    Ok(envs)
}

fn cargo_llvm_cov_fini(
    cargo: &Cargo,
    // workspace: &Metadata,
    // envs: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
) -> eyre::Result<()> {
    cargo.spawn([
        "llvm-cov",
        "--no-run",
        "--lcov",
        "--output-path",
        "lcov-xtask-test.info",
    ])
}