use clap::Parser;

#[derive(Parser)]
struct Cli {}

fn main() {
    let args = Cli::parse();
}
