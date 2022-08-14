use cargo_metadata::Target;
feature_clap_command! {
    use cargo_metadata::Package;
}

/// Configures and constructs [`TargetConfig`]
#[derive(Debug)]
pub struct TargetConfigBuilder<'a> {
    name: String,
    target: &'a Target,
    #[cfg(any(feature = "command-build-man", feature = "command-build-completion"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "command-build-man", feature = "command-build-completion")))
    )]
    command: Option<clap::Command<'static>>,
}

impl<'a> TargetConfigBuilder<'a> {
    pub(crate) fn from_metadata(target: &'a Target) -> Self {
        Self {
            name: target.name.clone(),
            target,
            #[cfg(any(feature = "command-build-man", feature = "command-build-completion"))]
            #[cfg_attr(
                docsrs,
                doc(cfg(any(
                    feature = "command-build-man",
                    feature = "command-build-completion"
                )))
            )]
            command: None,
        }
    }

    feature_clap_command! {
        pub(crate) fn binary_from_command(
            command: clap::Command<'static>,
            package: &'a Package,
        ) -> eyre::Result<Self> {
            let name = command.get_name().to_string();
            let target = package
                .targets
                .iter()
                .find(|t| t.name == name && t.kind.iter().any(|k| k == "bin"))
                .ok_or_else(|| eyre::eyre!("command not found: {name}"))?;
            Ok(Self {
                name,
                target,
                command: Some(command),
            })
        }
    }

    /// Builds a [`TargetConfig`] from the current configuration.
    pub fn build(self) -> TargetConfig<'a> {
        TargetConfig {
            name: self.name,
            target: self.target,
            #[cfg(any(feature = "command-build-man", feature = "command-build-completion"))]
            #[cfg_attr(
                docsrs,
                doc(cfg(any(
                    feature = "command-build-man",
                    feature = "command-build-completion"
                )))
            )]
            command: self.command,
        }
    }
}

/// Configuration for the distribution of the target.
#[derive(Debug)]
pub struct TargetConfig<'a> {
    name: String,
    target: &'a Target,
    #[cfg(any(feature = "command-build-man", feature = "command-build-completion"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "command-build-man", feature = "command-build-completion")))
    )]
    command: Option<clap::Command<'static>>,
}

impl<'a> TargetConfig<'a> {
    /// Return the name of the target.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the metadata of the target.
    pub fn target(&self) -> &Target {
        self.target
    }

    feature_clap_command! {
        /// Returns the [`clap::Command`] for the target.
        pub fn command(&self) -> Option<&clap::Command<'static>> {
            self.command.as_ref()
        }
    }
}
