use clap::Parser;

use crate::DistConfig;

/// `dist-build-license` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistBuildLicense {}

impl DistBuildLicense {
    /// Execute `dist-build-license` subcommand workflow
    #[tracing::instrument(name = "dist-build-license", parent = None, skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        use once_cell::sync::Lazy;
        use regex::{Regex, RegexBuilder};

        tracing::info!("Building license files...");

        let Self {} = self;

        for package in config.packages() {
            let src_dir = package.root_dir();
            let dest_dir = config
                .dist_working_directory()
                .join("share/license")
                .join(package.name());
            crate::fs::create_or_cleanup_dir(&dest_dir)?;

            if let Some(files) = package.license_files() {
                for file in files {
                    let src_file = src_dir.join(file);
                    let dest_file = dest_dir.join(file);
                    crate::fs::copy(&src_file, &dest_file)?;
                }
                continue;
            }

            for src_entry in src_dir.read_dir_utf8()? {
                let src_entry = src_entry?;
                if !src_entry.file_type()?.is_file() {
                    continue;
                }
                let src_file = src_entry.path();
                static RE: Lazy<Regex> = Lazy::new(|| {
                    RegexBuilder::new(r"^LICENSE(?:-|\.|$)")
                        .case_insensitive(true)
                        .build()
                        .unwrap()
                });
                let src_name = match src_file.file_name() {
                    Some(name) => name,
                    None => continue,
                };
                if !RE.is_match(src_name) {
                    continue;
                }
                let dest_file = dest_dir.join(src_name);
                crate::fs::copy(src_file, dest_file)?;
            }
        }

        Ok(())
    }
}
