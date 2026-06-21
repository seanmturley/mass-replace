use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mass-replace")]
#[command(about = "Performs mass character replacements in text files")]
struct Args {
    /// Input text file to transform
    #[arg(short, long)]
    input: String,

    /// Reference file with character mappings
    #[arg(short, long)]
    map: String,
}

fn main() {
    let _args = Args::parse();
    println!("Hello, world!");
}
