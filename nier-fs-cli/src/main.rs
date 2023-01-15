use clap::Parser;
use clap::Subcommand;
use std::fs::File;
use nier_fs::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A CLI for working with NieR: Automata files")]
struct Config {
    // Pack or unpack
    // Note: this is optional and default behavior is:
    //  - pack folder
    //  - unpack file
    /// hi
    #[command(subcommand)]
    action: Option<Action>,
    
    /// Path to file or directory
    #[arg(short, long)]
    path: String,

    /// Recurse subdirectories
    #[arg(short, long)]
    recursive: bool,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Pack directory into container (dir -> CPK/DAT)
    Pack,

    /// Unpack container into directory (CPK/DAT -> dir)
    Unpack
}

fn main() {
    let mut output_path = std::env::current_dir().unwrap_or_else(|error| {
        panic!("Problem getting current directory: {:?}", error);
    });

    output_path.push("output");

    let config = Config::parse();

    let mut file = File::open(config.path).unwrap_or_else(|error| {
        panic!("Problem opening the input file: {:?}", error);
    });

    dat::unpack(file, &output_path).unwrap_or_else(|error| {
        panic!("Problem unpacking the input file: {:?}", error)
    });

}
