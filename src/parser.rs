use std::io::BufRead;

use anyhow::{anyhow, bail, Result};

use crate::vec3::Vec3;

pub fn parse<R>(
    input: R,
    points: &mut Vec<Vec3>,
    roots: &mut Vec<Vec3>,
) -> Result<()>
where
    R: BufRead,
{
    let mut lines = input.lines().map(|a| Ok(a?));

    read_first_points(&mut lines, points)?;
    read_first_points(&mut lines, roots)?;

    Ok(())
}

fn read_first_points<I>(input: &mut I, points: &mut Vec<Vec3>) -> Result<()>
where
    I: Iterator<Item = Result<String>>,
{
    while let Some(l) = input.next() {
        let l = l?;
        if l.starts_with('(') {
            return read_all_points(&l, points);
        }
    }

    bail!("No line with points.");
}

fn read_all_points(mut s: &str, res: &mut Vec<Vec3>) -> Result<()> {
    s = s.trim_start();
    if s.is_empty() || s.starts_with('.') {
        return Ok(());
    }

    fn expect(mut s: &str, chr: char) -> Result<&str> {
        s = s.trim_start();
        if !s.starts_with(chr) {
            bail!("Expected '{chr}'");
        }
        return Ok(&s[1..]);
    }

    fn read_num(s: &mut &str) -> Result<f64> {
        *s = s.trim_start();
        let Some((n, ns)) =
            s.split_once(|c: char| !c.is_digit(10) && c != '.' && c != '-')
        else {
            let res = s.parse()?;
            *s = "";
            return Ok(res);
        };

        *s = ns;
        Ok(n.parse()?)
    }

    s = expect(s, '(')?;

    let x = read_num(&mut s)?;
    s = expect(s, ',').unwrap_or(s);
    let y = read_num(&mut s)?;
    s = expect(s, ',').unwrap_or(s);
    let z = read_num(&mut s)?;
    s = expect(s, ')').unwrap_or(s);

    res.push(Vec3(x, y, z));
    read_all_points(s, res)
}
