use clap::Parser;
use clap::Subcommand;
use std::fs::File;
use std::path::{Path, PathBuf};
use nier_fs::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A CLI for working with NieR: Automata files")]
struct Config {
    // Pack or unpack
    // Note: this is optional and default behavior is:
    //  - pack folder
    //  - unpack file
    #[command(subcommand)]
    action: Option<Action>,
    
    /// Path to input file or directory
    #[arg(short, long)]
    input: String,
    
    /// Path to output file or directory (inside input dir or next to input file by default)
    #[arg(short, long)]
    output: Option<String>,

    /// Recurse subdirectories
    #[arg(short, long)]
    recursive: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Pack directory into container (dir -> CPK/DAT)
    Pack,

    /// Unpack container into directory (CPK/DAT -> dir)
    Unpack
}

fn main() {
    let config = Config::parse();
    let input_path = Path::new(&config.input);
    let output_path: PathBuf;

    // By default, output_path is the input_path
    match config.output {
        Some(path) => {
            output_path = PathBuf::from(path);
        }

        None => {
            let parent = input_path.parent().unwrap().to_str().unwrap();
            let stem = input_path.file_stem().unwrap();
            let extension = input_path.extension().unwrap();
            let mut path_str = String::from(parent);
            path_str.push('/');
            path_str.push_str(stem.to_str().unwrap());
            path_str.push('_');
            path_str.push_str(extension.to_str().unwrap());
            path_str.push('/');
            output_path = PathBuf::from(path_str);
        }
    }

    let file = File::open(config.input).unwrap_or_else(|error| {
        panic!("Problem opening the input file: {:?}", error);
    });

    dat::unpack(file, &output_path, config.verbose).unwrap_or_else(|error| {
        panic!("Problem unpacking the input file: {:?}", error)
    });
}