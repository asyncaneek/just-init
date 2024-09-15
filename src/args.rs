use std::path::PathBuf;

use crate::{data::Data, output::Output, source::Source};

/// A program to generate directories and files using jinja templates
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to template file or folder
    #[arg(short, long, value_parser=clap::value_parser!(PathBuf))]
    pub source: Source,

    /// Path to json file containing jinja variables
    #[arg(short, long, value_parser=clap::value_parser!(PathBuf))]
    pub data: Data,

    /// Path to output folder
    #[arg(short, long, value_parser=clap::value_parser!(PathBuf), default_value = ".")]
    pub output: Output,
}
