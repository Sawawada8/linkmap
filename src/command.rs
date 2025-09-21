use anyhow::Error;
use clap::Parser;
use reqwest::blocking::Client;
use url::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    // #[arg(short, long)]
    url: Url,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    depth: u8,
}

pub fn exec() -> Result<(), Error> {
    let arg = Args::parse();
    let client = Client::new();
    let res = client.get(arg.url).send().unwrap();
    let body = res.text().unwrap();
    println!("{:?}", body);

    Ok(())
}
