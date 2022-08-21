use crate::{config::Config, Result, Run};

/// `dist-build-doc` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct DistBuildReadme {}

impl Run for DistBuildReadme {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl DistBuildReadme {
    /// Execute `dist-build-doc` subcommand workflow.
    #[tracing::instrument(name = "dist-build-readme", parent = None, skip_all, err)]
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
                let src_file = package.root_dir().join(readme);
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
