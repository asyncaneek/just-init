use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use super::*;

use anyhow::Result;
use args::Args;
use tera::{Context, Tera};

pub struct Engine;

#[derive(Default)]
pub struct Report {
    rendered_success: Vec<PathBuf>,
    // ( template, error message )
    rendered_failures: Vec<(String, String)>,
}

impl Engine {
    pub fn render_to_disk(args: Args) -> Result<Report> {
        // load all files from template
        let tera = Tera::new(
            &args
                .source
                .dir
                .canonicalize()?
                .join(args.source.pattern)
                .display()
                .to_string(),
        )?;
        let output_dir = args.output.as_path();
        let context = Context::from_serialize(args.data)?;

        let mut report = Report::default();
        for name in tera.get_template_names() {
            match generate(name, &context, output_dir, &tera) {
                Ok(generated_filepath) => {
                    report.rendered_success.push(generated_filepath);
                }
                Err(err) => {
                    report
                        .rendered_failures
                        .push((name.to_string(), err.to_string()));
                }
            }
        }

        Ok(report)
    }
}

// filepath_as_str also doubles as the tera template name
fn generate(
    filepath_as_str: &str,
    context: &Context,
    output: &Path,
    tera: &Tera,
) -> Result<PathBuf> {
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
