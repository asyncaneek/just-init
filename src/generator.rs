use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use super::*;

use anyhow::Result;
use data::Data;
use output::Output;
use source::Source;
use tera::{Context, Tera};

pub struct Generator {
    context: Context,
}

#[derive(Debug, Default)]
pub struct Report {
    pub rendered_success: Vec<PathBuf>,
    // ( template, error message )
    pub failures: Vec<(String, String)>,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for success in &self.rendered_success {
            write!(f, "Generated: {}", success.display())?;
        }

        for (templ, err) in &self.failures {
            write!(f, "Failed to Generate: {} \n because {}", &templ, &err)?;
        }

        Ok(())
    }
}

impl Generator {
    pub fn new(data: Data) -> Result<Self> {
        let context = Context::from_serialize(data)?;
        Ok(Self { context })
    }

    pub fn render_to_disk(&self, source: Source, output: Output) -> Result<Report> {
        // load all files from template
        let tera = Tera::new(
            &source
                .dir
                .canonicalize()?
                .join(source.pattern)
                .display()
                .to_string(),
        )?;
        let output_dir = output.as_path();

        let mut report = Report::default();
        for name in tera.get_template_names() {
            match generate(name, &self.context, output_dir, &tera) {
                Ok(generated_filepath) => {
                    report.rendered_success.push(generated_filepath);
                }
                Err(err) => {
                    report.failures.push((name.to_string(), err.to_string()));
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
