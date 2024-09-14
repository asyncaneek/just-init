use super::*;

use std::path::PathBuf;

use crate::data::Data;

/// A program to generate directories and files using jinja templates
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to template file or folder
    #[arg(short, long)]
    template: PathBuf,

    /// Path to json file containing jinja variables
    #[arg(short, long)]
    data: PathBuf,

    /// Path to output folder
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}

pub trait Parser {
    fn parse_args(args: Args) -> Result<(Source, Data, Output)>;
}