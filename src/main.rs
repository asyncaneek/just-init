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
    #[arg(short, long, conflicts_with = "inline_data")]
    data_file: Option<PathBuf>,

    /// Inline key-value pairs for jinja variables (format: key=value)
    #[arg(short = 'i',
        long,
        value_parser = parse_key_val,
        num_args = 1..,
        value_delimiter = ' ',
        conflicts_with = "data_file")]
    inline_data: Option<Vec<(String, String)>>,

    /// Path to output folder
    #[arg(short, long)]
    output: PathBuf,
}

/// Parse a single key-value pair
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn main() -> Result<()> {
    let args = <Args as clap::Parser>::parse();

    let data = if let Some(data_file) = args.data_file {
        Data::try_from(data_file.as_path())?
    } else if let Some(inline_data) = args.inline_data {
        Data::try_from(inline_data)?
    } else {
        return Err(anyhow::anyhow!(
            "Either --data or --inline-data must be provided"
        ));
    };

    let source = Source::try_from(args.source.as_path())?;
    let output = Output::try_from(args.output.as_path())?;
    let generator = Generator::new(data)?;

    let report = generator.render_to_disk(source, output)?;
    println!("Report: {}", report);

    Ok(())
}
