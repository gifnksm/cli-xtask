fn main() -> eyre::Result<()> {
    #[cfg(feature = "main")]
    {
        use cli_xtask::Xtask;
        <Xtask>::main()?;
    }

    Ok(())
}
