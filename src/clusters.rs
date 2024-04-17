use crate::vec3::Vec3;

pub fn make_clusters(
    roots: Vec<Vec3>,
    points: Vec<Vec3>,
) -> Vec<(Vec3, Vec<Vec3>)> {
    let mut res: Vec<_> = roots.into_iter().map(|r| (r, vec![])).collect();
    for p in points {
        let idx = closest(p, &res);
        res[idx].1.push(p);
    }

    let mut iteration = 0;
    print_clusters(iteration, &res);
    iteration += 1;

    let mut moved = true;
    while moved {
        for g in res.iter_mut() {
            g.0 = g.1.iter().cloned().sum::<Vec3>() / g.1.len() as f64;
        }

        moved = false;
        for gi in 0..res.len() {
            let mut pi = 0;
            while pi < res[gi].1.len() {
                let p = res[gi].1[pi];
                let ci = closest(p, &res);
                if ci != gi {
                    moved = true;
                    res[gi].1.remove(pi);
                    res[ci].1.push(p);
                } else {
                    pi += 1;
                }
            }
        }

        print_clusters(iteration, &res);
        iteration += 1;
    }

    res
}

fn closest(point: Vec3, choose: &[(Vec3, Vec<Vec3>)]) -> usize {
    choose
        .iter()
        .enumerate()
        .min_by(|(_, (a, _)), (_, (b, _))| {
            (*a - point).len().total_cmp(&(*b - point).len())
        })
        .unwrap()
        .0
}

fn print_clusters(i: usize, clusters: &Vec<(Vec3, Vec<Vec3>)>) {
    println!("Iterace {i}:");
    for (m, c) in clusters {
        println!("  {m}:");
        print!("    ");
        for v in c {
            print!("{v} ");
        }
        println!();
    }
    println!();
}
