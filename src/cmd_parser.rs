use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(
    author = "Adam Buckingham",
    version,
    about = "Start to visualize some message structs"
)]
pub struct PathVisualizerArgs {
    /// The path to the root file structure
    pub root_path: std::path::PathBuf,
    /// The path to the file to read
    pub msg_path: std::path::PathBuf,
    /// The message name to look for
    pub msg_name: String,
}
