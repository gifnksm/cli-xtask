use std::any::Any;

use crate::{config::Config, Result, Run, SubcommandRun};

/// Arguments definition of the `dist-build-readme` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-build-readme.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistBuildReadme {}

impl Run for DistBuildReadme {
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

impl DistBuildReadme {
    /// Runs the `dist-build-doc` subcommand.
    #[tracing::instrument(name = "dist-build-readme", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        tracing::info!("Building READMEs...");

        let Self {} = self;
        let config = config.dist()?;

        let packages = config.packages();

        let working_dir = config.dist_working_directory(None);
        let readme_dir;
        let add_package_dir;
        if packages.len() > 1 {
            readme_dir = working_dir.join("readme");
            add_package_dir = true;
            crate::fs::remove_dir(&readme_dir)?;
        } else {
            readme_dir = working_dir;
            add_package_dir = false;
        }

        let Self {} = self;

        for package in packages {
            if let Some(readme) = &package.metadata().readme {
                let src_file = package.root_directory().join(readme);
                let dest_dir = if add_package_dir {
                    readme_dir.join(package.name())
                } else {
                    readme_dir.clone()
                };
                let dest_file = dest_dir.join(readme);
                crate::fs::copy(&src_file, &dest_file)?;
            }
        }

        Ok(())
    }
}
