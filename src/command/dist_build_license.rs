use eyre::eyre;

use crate::{config::Config, fs::ToRelative, Result, Run};

/// `dist-build-license` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct DistBuildLicense {}

impl Run for DistBuildLicense {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl DistBuildLicense {
    /// Execute `dist-build-license` subcommand workflow
    #[tracing::instrument(name = "dist-build-license", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        tracing::info!("Building license files...");

        let Self {} = self;
        let config = config.dist()?;

        let packages = config.packages();

        let working_dir = config.dist_working_directory(None);
        let license_dir;
        let add_package_dir;
        if packages.len() > 1 {
            license_dir = working_dir.join("license");
            add_package_dir = true;
            crate::fs::remove_dir(&license_dir)?;
        } else {
            license_dir = working_dir;
            add_package_dir = false;
        }

        for package in config.packages() {
            let dest_dir = if add_package_dir {
                license_dir.join(package.name())
            } else {
                license_dir.clone()
            };

            for src_file in package.license_files() {
                let file_name = src_file
                    .file_name()
                    .ok_or_else(|| eyre!("license file has no name: {}", src_file.to_relative()))?;
                let dest_file = dest_dir.join(file_name);
                crate::fs::copy(&src_file, &dest_file)?;
            }
        }

        Ok(())
    }
}
