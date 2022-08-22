fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let command_enabled = std::env::vars()
        .any(|(name, var)| name.starts_with("CARGO_FEATURE_SUBCOMMAND_") && var == "1");
    if command_enabled {
        println!("cargo:rustc-cfg=subcommand");
    }

    let command_build_enabled = std::env::vars()
        .any(|(name, var)| name.starts_with("CARGO_FEATURE_SUBCOMMAND_DIST_BUILD_") && var == "1");
    if command_build_enabled {
        println!("cargo:rustc-cfg=subcommand_dist_build");
    }

    println!(
        "cargo:rustc-env=DEFAULT_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
}
