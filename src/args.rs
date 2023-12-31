use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args{
    //The ROM file to execute.
    pub rom_file: PathBuf
}
