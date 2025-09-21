mod command;

fn main() -> Result<(), anyhow::Error> {
    command::exec().unwrap();
    println!("Hello, world!");

    Ok(())
}
