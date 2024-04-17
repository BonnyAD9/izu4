use std::io;

use anyhow::Result;
use clusters::make_clusters;
use parser::parse;

mod clusters;
mod parser;
mod vec3;

fn main() -> Result<()> {
    let mut points = vec![];
    let mut roots = vec![];

    parse(io::stdin().lock(), &mut points, &mut roots)?;

    make_clusters(roots, points);

    Ok(())
}
