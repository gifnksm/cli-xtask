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
            #[cfg(command_build)]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-build-*")))]
            $item
        )*
    };
}

macro_rules! feature_command_build_bin {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-build-bin")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-build-bin")))]
            $item
        )*
    }
}

macro_rules! feature_command_build_completion {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-build-completion")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-build-completion")))]
            $item
        )*
    };
}

macro_rules! feature_command_build_doc {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-build-doc")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-build-doc")))]
            $item
        )*
    };
}

macro_rules! feature_command_build_license {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-build-license")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-build-license")))]
            $item
        )*
    };
}

macro_rules! feature_command_build_man {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "command-build-man")]
            #[cfg_attr(docsrs, doc(cfg(feature = "command-build-man")))]
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

macro_rules! feature_clap_command {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "command-build-man", feature = "command-build-completion"))]
            #[cfg_attr(
                docsrs,
                doc(cfg(any(
                    feature = "command-build-man",
                    feature = "command-build-completion"
                )))
            )]
            $item
        )*
    };
}
