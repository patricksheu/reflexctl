fn main() {
    println!("Hello, world!");
}
use clap::Parser;
use reflexctl::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()
}
