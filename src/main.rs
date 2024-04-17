use std::{fs::File, io::BufReader};

use anyhow::Result;
use clusters::make_clusters;
use parser::parse;

mod clusters;
mod parser;
mod vec3;

fn main() -> Result<()> {
    let mut points = vec![];
    let mut roots = vec![];
    let file = BufReader::new(File::open("xstigl00.txt")?);

    parse(file, &mut points, &mut roots)?;

    let clusters = make_clusters(roots, points);

    Ok(())
}
