use std::io::Write;

const GRID_SEED: u64 = 0;
const GRID_SIZE: arc::codec::Scalar = 100;
const GRID_DENSITY: f64 = 0.5;

const COLOR_COUNT: arc::codec::Scalar = 200;

fn count(edges: & [Box<[arc::codec::Scalar]>]) -> usize
{
    let mut count = 0;

    for (node, targets) in edges.iter().enumerate()
    {
        let node = node as arc::codec::Scalar;

        for target in targets.iter().copied()
        {
            if target <= node
            {
                continue;
            };

            count += 1;
        };
    };

    return count;
}

#[derive(clap::Parser)]
struct Args
{
    #[arg(long)]
    path: std::path::PathBuf,
    #[arg(long)]
    tick: arc::codec::Scalar
}

fn main()
{
    let args = <Args as clap::Parser>::parse();

    println!("tick: {} path: {}", args.tick, args.path.display());

    let seed = GRID_SEED;

    let size = GRID_SIZE * args.tick;

    let density = GRID_DENSITY;

    let color_count = COLOR_COUNT;

    let mut rng = <rand::rngs::Xoshiro256PlusPlus as rand::SeedableRng>::seed_from_u64(seed);

    let edges = color::utility::get_edges(& mut rng, size, density);

    let edge_count = count(& edges);

    let file = std::fs::File::create(args.path).unwrap();

    let mut writer = std::io::BufWriter::new(file);

    std::writeln!(writer, "n = {size};").unwrap();
    std::writeln!(writer, "k = {color_count};").unwrap();
    std::writeln!(writer, "m = {edge_count};").unwrap();
    std::writeln!(writer, "edges = array2d(1..m, 1..2, [").unwrap();

    let mut written = 0;

    for (node, targets) in edges.iter().enumerate()
    {
        let node = node as arc::codec::Scalar;

        for target in targets.iter().copied()
        {
            if target <= node
            {
                continue;
            };

            written += 1;

            let comma = if written == edge_count { "" } else { "," };

            std::writeln!(writer, "  {}, {}{}", node + 1, target + 1, comma).unwrap();
        };
    };

    std::writeln!(writer, "]);").unwrap();
}