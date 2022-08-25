use crate::{cargo, config::Config, Result, Run};

/// Arguments definition of the `dist-build-bin` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-build-bin.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
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

impl Run for DistBuildBin {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl DistBuildBin {
    /// Runs the `dist-build-bin` subcommand.
    #[tracing::instrument(name = "dist-build-bin", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        tracing::info!("Building executables...");

        let Self {
            target_triple,
            use_cross,
            use_cross_if_needed,
        } = self;
        let config = config.dist()?;

        let default_target = env!("DEFAULT_TARGET");
        let target_triple = target_triple.as_deref();
        let use_cross = *use_cross
            || (*use_cross_if_needed
                && target_triple.map(|t| t != default_target).unwrap_or(false));

        let bin_dir = config.dist_working_directory(Some(target_triple.unwrap_or(default_target)));
        crate::fs::create_or_cleanup_dir(&bin_dir)?;

        for package in config.packages() {
            for target in package.targets() {
                let artifacts = cargo::build(
                    config.metadata(),
                    Some(package.metadata()),
                    Some(target.metadata()),
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
