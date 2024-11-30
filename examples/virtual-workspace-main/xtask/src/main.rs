use cli_xtask::{
    config::{ConfigBuilder, DistConfigBuilder, DistPackageConfigBuilder},
    workspace, Result, Xtask,
};

fn main() -> Result<()> {
    <Xtask>::main_with_config(|| {
        let workspace = workspace::current();

        let (dist_config, pkg_configs) =
            DistConfigBuilder::from_default_packages("app-v0.1.0", workspace);
        let pkg_configs = pkg_configs
            .into_iter()
            .map(DistPackageConfigBuilder::build)
            .collect::<Result<Vec<_>>>()?;
        let dist_config = dist_config.packages(pkg_configs).build()?;
        let config = ConfigBuilder::new().dist(dist_config).build()?;
        Ok(config)
    })
}
