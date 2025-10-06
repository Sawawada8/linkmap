mod command;
mod scored_url;

fn main() -> Result<(), anyhow::Error> {
    command::exec()?;

    Ok(())
}
