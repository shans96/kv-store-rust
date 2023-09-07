use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CommandLineInterface {
    /// Operation to perform on the key/value store. Can be `set`, `get`, or `rm`.
    command: String
}

fn main() {
    let args = CommandLineInterface::parse();
}
