use clap::Parser;

#[derive(Parser)]
struct Commands {
    config: Option<String>,
}

fn main() {
    Commands::parse();
}
