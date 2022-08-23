/// Extract subcommands from clap's help message
pub fn subcommands_from_help(help: &str) -> Vec<&str> {
    help.lines()
        .skip_while(|l| !l.starts_with("SUBCOMMANDS:"))
        .skip(1)
        .take_while(|l| l.starts_with("    "))
        .filter_map(|l| {
            let cmd = l.strip_prefix("    ")?.split_once(' ')?.0;
            (!cmd.is_empty()).then(|| cmd)
        })
        .collect()
}
