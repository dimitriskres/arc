use core::panic;

const SIZE: arc::codec::Scalar = 100;

type Audit<Model> = chain::method::v0::audit::Audit<Model>;

type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

type Queue<Model> = arc::assert::queue::QueueV3<Model, bitset::meta::BitSet>;

type Cache<Model> = arc::assert::cache::CacheV4<Model, bitset::flat::BitSet>;

type Probe<Model> = arc::coerce::probe::ProbeV2<Model>;

#[derive(clap::Parser)]
struct Args
{
    #[arg(long)]
    tick: arc::codec::Scalar,
    #[arg(long)]
    version: usize
}

fn main()
{
    let args = <Args as clap::Parser>::parse();

    if args.version != 0
    {
        panic!("only version 0 is supported");
    };

    let instant = std::time::Instant::now();

    let size = SIZE * args.tick;

    let node_count = size;

    let unit_count = size;

    let model = & chain::method::v0::model::ScalarModel::new(node_count, unit_count);

    let report = arc::analyze::solve::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model);

    let total_duration = instant.elapsed().as_nanos().to_string();
    
    let solve_duration = report.duration.as_nanos().to_string();

    println!("{:},{:}", solve_duration, total_duration);
}