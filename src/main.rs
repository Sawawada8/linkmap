mod command;

fn main() -> Result<(), anyhow::Error> {
    command::exec()?;
    println!("Hello, world!");

    Ok(())
}
