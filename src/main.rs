use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use tera::Context;
use tera::Tera;

/// A program to generate directories and files using jinja templates
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to template file or folder
    #[arg(short, long)]
    template: PathBuf,

    /// Path to json file containing jinja variables
    #[arg(short, long)]
    data: PathBuf,

    /// Path to output folder
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}// change

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);

    // load all files from template
    let template = args.template.as_path().display().to_string() + "/**/*";
    
    let output = args.output.as_path();
    
    let context = load_context_from_json(&args.data)?;

    let tera = Tera::new(&template)?;

    for name in tera.get_template_names() {
        let generated_filepath = generate(name, &context, output, &tera)?;

        println!("Generated: {:#?}", &generated_filepath);
    }

    Ok(())
}

// filepath_as_str also doubles as the tera template name
fn generate(filepath_as_str: &str, context: &Context, output: &Path, tera: &Tera) -> Result<PathBuf, Error> {
    // generate folders
    let path = Tera::one_off(filepath_as_str, context, false)?;
    let path_to_file = output.join(path);
    let mut folders_to_create = path_to_file.clone();
    folders_to_create.pop();
    fs::create_dir_all(folders_to_create)?;

    // generate file
    let mut file = File::create(&path_to_file)?;
    let content = tera.render(filepath_as_str, context)?;
    file.write_all(content.as_bytes())?;

    Ok(path_to_file.canonicalize()?)
}

// parse json file into tera context
fn load_context_from_json(json_file_path: &Path) -> Result<Context> {
    let content = std::fs::read_to_string(json_file_path)?;
    let json_value = serde_json::from_str(&content)?;
    let context = Context::from_value(json_value)?;
    Ok(context)
}
