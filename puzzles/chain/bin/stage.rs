use std::io::Write;

const SIZE: arc::codec::Scalar = 100;

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

    let size = SIZE * args.tick;

    let file = std::fs::File::create(args.path).unwrap();

    let mut writer = std::io::BufWriter::new(file);

    std::writeln!(writer, "n = {size};").unwrap();
}