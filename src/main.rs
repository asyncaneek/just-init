use anyhow::Result;
use just_init::args::{Args, Parser};
use just_init::engine::Engine;

fn main() -> Result<()> {
    let args = <Args as clap::Parser>::parse();
    let (source, parameters, output) = Engine::parse_args(args)?;
    Engine::render_to_disk(source, parameters, output)?;

    Ok(())
}