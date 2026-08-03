use std::io::Write;

const SIZE: arc::codec::Scalar = 2;

#[derive(clap::Parser)]
struct Args
{
    #[arg(long)]
    path: std::path::PathBuf,
    #[arg(long)]
    tick: arc::codec::Scalar
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = <Args as clap::Parser>::parse();

    let size = 10 + SIZE * args.tick;

    let file = std::fs::File::create(args.path)?;

    let mut writer = std::io::BufWriter::new(file);

    std::writeln!(writer, "n = {size};")?;

    return Ok(());
}