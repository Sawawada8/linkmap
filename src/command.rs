use clap::Parser;
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
    count: u8,
}

pub fn a() {
    let arg = Args::parse();
    println!("{:?}", arg);
}
