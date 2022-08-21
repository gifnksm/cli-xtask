use eyre::eyre;

use crate::{config::Config, fs::ToRelative, Result, Run};

/// `dist-build-doc` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistBuildDoc {}

impl Run for DistBuildDoc {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl DistBuildDoc {
    /// Execute `dist-build-doc` subcommand workflow.
    #[tracing::instrument(name = "dist-build-doc", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        tracing::info!("Building documents...");

        let Self {} = self;
        let config = config.dist()?;

        let packages = config.packages();

        let working_dir = config.dist_working_directory(None);
        let doc_dir = working_dir.join("doc");
        let add_package_dir = packages.len() > 1;
        crate::fs::remove_dir(&doc_dir)?;

        let Self {} = self;

        for package in packages {
            let dest_dir = if add_package_dir {
                doc_dir.join(package.name())
            } else {
                doc_dir.clone()
            };

            for src_file in package.documents() {
                let file_name = src_file.file_name().ok_or_else(|| {
                    eyre!("document file has no name: {}", src_file.to_relative())
                })?;
                let dest_file = dest_dir.join(file_name);
                crate::fs::copy(&src_file, &dest_file)?;
            }
        }

        Ok(())
    }
}
