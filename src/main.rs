mod cmd_parser;
use clap::Parser;
use cmd_parser::PathVisualizerArgs;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let cli_args = PathVisualizerArgs::parse();
    println!("CLI Arguments -> {:?}", &cli_args);
    println!("Root Path: {}", &cli_args.root_path.display());
    println!("File Path: {}", &cli_args.msg_path.display());
    println!("Message: {}", &cli_args.msg_name);

    //let readable_file_path = cli_args.msg_path.as_path();

    reading_file(cli_args.msg_path)
}

fn reading_file(path: std::path::PathBuf) {
    let file = File::open(path).expect("issue opening the file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();
        println!("{:?}", l);
    }
}
