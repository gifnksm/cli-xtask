fn main() {
    let command_enabled = std::env::vars()
        .any(|(name, var)| name.starts_with("CARGO_FEATURE_COMMAND_") && var == "1");
    if command_enabled {
        println!("cargo:rustc-cfg=command");
    }

    let command_build_enabled = std::env::vars()
        .any(|(name, var)| name.starts_with("CARGO_FEATURE_COMMAND_DIST_BUILD_") && var == "1");
    if command_build_enabled {
        println!("cargo:rustc-cfg=command_dist_build");
    }

    println!(
        "cargo:rustc-env=DEFAULT_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
}
