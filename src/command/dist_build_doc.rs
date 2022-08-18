use clap::Parser;

use crate::config::Config;

/// `dist-build-doc` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistBuildDoc {}

impl DistBuildDoc {
    /// Execute `dist-build-doc` subcommand workflow.
    #[tracing::instrument(name = "dist-build-doc", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> eyre::Result<()> {
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
            let src_dir = package.root_dir();
            let dest_dir = if add_package_dir {
                doc_dir.join(package.name())
            } else {
                doc_dir.clone()
            };

            if let Some(files) = package.documents() {
                for file in files {
                    let src_file = src_dir.join(file);
                    let dest_file = dest_dir.join(file);
                    crate::fs::copy(&src_file, &dest_file)?;
                }
                continue;
            }
        }

        Ok(())
    }
}
