use clap::Parser;

use crate::{cargo, DistConfig};

/// `dist-build-bin` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistBuildBin {
    /// Target triple for the build
    #[clap(long = "target")]
    pub target_triple: Option<String>,
    /// Use cross tool to build
    #[clap(long)]
    pub use_cross: bool,
    /// Use cross if target is different from default target
    #[clap(long)]
    pub use_cross_if_needed: bool,
}

impl DistBuildBin {
    /// Execute `dist-build-bin` subcommand workflow.
    #[tracing::instrument(name = "dist-build-bin", skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        tracing::info!("Building executables...");

        let Self {
            target_triple,
            use_cross,
            use_cross_if_needed,
        } = self;

        let target_triple = target_triple.as_deref();
        let use_cross = *use_cross
            || (*use_cross_if_needed
                && target_triple
                    .map(|t| t != env!("DEFAULT_TARGET"))
                    .unwrap_or(false));

        let bin_dir = config.dist_working_directory().join("bin");
        crate::fs::create_or_cleanup_dir(&bin_dir)?;

        for package in config.packages() {
            for target in package.targets().into_iter().flatten() {
                let artifacts = cargo::build(
                    config.metadata(),
                    Some(package.package()),
                    Some(target.target()),
                    Some("release"),
                    use_cross,
                    target_triple,
                )?;
                for src in artifacts {
                    let src = src?;
                    let dest = bin_dir.join(target.name());
                    crate::fs::copy(&src, &dest)?;
                }
            }
        }

        Ok(())
    }
}
