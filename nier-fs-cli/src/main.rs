use clap::Parser;
use std::fs::File;
use nier_fs::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A CLI for working with NieR: Automata files")]
struct Config {
    // Path to file
    #[arg(short, long)]
    path: String,
}

fn main() {
    let mut output_path = std::env::current_dir().unwrap_or_else(|error| {
        panic!("Problem getting current directory: {:?}", error);
    });

    output_path.push("output");

    let config = Config::parse();
    println!("{:#?}", config.path);

    let mut file = File::open(config.path).unwrap_or_else(|error| {
        panic!("Problem opening the input file: {:?}", error);
    });

    dat::unpack(file, &output_path).unwrap_or_else(|error| {
        panic!("Problem unpacking the input file: {:?}", error)
    });

}
