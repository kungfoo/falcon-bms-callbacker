use vergen_git2::*;

fn main() -> Result<(), anyhow::Error> {
    let git2 = Git2Builder::default().describe(true, true, None).build()?;

    Emitter::default().add_instructions(&git2)?.emit()?;

    Ok(())
}
