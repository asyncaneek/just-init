use super::*;

use anyhow::Result;
use tera::Context;

pub struct Engine {
    context: Context,
}

impl Engine {
    pub fn render_to_disk(source: Source, data: Data, output: Output) -> Result<()> {
        todo!()
    }
}

impl Parser for Engine {
    fn parse_args(args: Args) -> Result<(Source, Data, Output)> {
        todo!()
    }
}
