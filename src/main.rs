use std::path::PathBuf;

use anyhow::Result;
use data::Data;
use generator::Generator;
use output::Output;
use source::Source;

pub mod data;
pub mod generator;
pub mod output;
pub mod source;

/// A program to generate directories and files using jinja templates
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to template file or folder
    #[arg(short, long)]
    source: PathBuf,

    /// Path to json file containing jinja variables
    #[arg(short, long)]
    data: PathBuf,

    /// Path to output folder
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = <Args as clap::Parser>::parse();

    let data = Data::try_from(args.data.as_path())?;
    let source = Source::try_from(args.source.as_path())?;
    let output = Output::try_from(args.output.as_path())?;
    let generator = Generator::new(data)?;

    let report = generator.render_to_disk(source, output)?;
    println!("Report: {}", report);

    Ok(())
}
