#![allow(unused_macros)]

macro_rules! feature_main {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "main")]
            #[cfg_attr(docsrs, doc(cfg(feature = "main")))]
            $item
        )*
    };
}

macro_rules! feature_logger {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "logger")]
            #[cfg_attr(docsrs, doc(cfg(feature = "logger")))]
            $item
        )*
    };
}

macro_rules! feature_error_handler {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "error-handler")]
            #[cfg_attr(docsrs, doc(cfg(feature = "error-handler")))]
            $item
        )*
    };
}

macro_rules! feature_archive {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "archive")]
            #[cfg_attr(docsrs, doc(cfg(feature = "archive")))]
            $item
        )*
    };
}

macro_rules! feature_cargo {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "cargo")]
            #[cfg_attr(docsrs, doc(cfg(feature = "cargo")))]
            $item
        )*
    };
}

macro_rules! feature_command {
    ($($item:item)*) => {
        $(
            #[cfg(command)]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-*")))]
            $item
        )*
    };
}

macro_rules! feature_command_build {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-build")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-build")))]
            $item
        )*
    };
}

macro_rules! feature_command_clippy {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-clippy")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-clippy")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_archive {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-archive")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-archive")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_build {
    ($($item:item)*) => {
        $(
            #[cfg(command_dist_build)]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-*")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_build_bin {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-build-bin")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-bin")))]
            $item
        )*
    }
}

macro_rules! feature_command_dist_build_completion {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-build-completion")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-completion")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_build_doc {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-build-doc")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-doc")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_build_license {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-build-license")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-license")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_build_man {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-build-man")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-man")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_build_readme {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-build-readme")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-readme")))]
            $item
        )*
    };
}

macro_rules! feature_command_dist_clean {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-dist-clean")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-clean")))]
            $item
        )*
    };
}

macro_rules! feature_command_fmt {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-fmt")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-fmt")))]
            $item
        )*
    };
}

macro_rules! feature_command_rdme {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-rdme")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-rdme")))]
            $item
        )*
    };
}

macro_rules! feature_command_test {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-test")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-test")))]
            $item
        )*
    };
}

macro_rules! feature_clap_command {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "command-dist-build-man", feature = "command-dist-build-completion"))]
            #[cfg_attr(
                docsrs,
                doc(cfg(any(
                    feature = "command-dist-build-man",
                    feature = "command-dist-build-completion"
                )))
            )]
            $item
        )*
    };
}
