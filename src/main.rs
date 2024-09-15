use anyhow::Result;
use just_init::args::Args;
use just_init::engine::Engine;

fn main() -> Result<()> {
    let args = <Args as clap::Parser>::parse();

    Engine::render_to_disk(args)?;

    Ok(())
}
