
const SIZE: arc::codec::Scalar = 2;

type Probe<Model> = arc::coerce::probe::ProbeV2<Model>;

fn exec_v0(size: arc::codec::Scalar) -> arc::analyze::SolveReport
{
    type Audit<Model> = latin::method::v0::audit::Audit<Model>;

    type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

    type Queue<Model> = arc::assert::queue::QueueV1<Model>;

    type Cache<Model> = arc::assert::cache::CacheV4<Model, bitset::flat::BitSet>;

    let model = & latin::method::v0::model::ScalarModel::new(size);

    let report = arc::analyze::solve::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model);

    return report;
}

fn exec_v1(size: arc::codec::Scalar) -> arc::analyze::SolveReport
{
    type Audit<Model> = latin::method::v1::audit::Audit<Model>;

    type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

    type Queue<Model> = arc::assert::queue::QueueV1<Model>;

    type Cache<Model> = arc::assert::cache::CacheV5<Model, bitset::flat::BitSet>;

    let model = & latin::method::v1::model::ScalarModel::new(size);

    let report = arc::analyze::solve::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model);

    return report;
}


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

    let instant = std::time::Instant::now();

    let size = 10 + SIZE * args.tick;

    let report = match args.version
    {
        0 => exec_v0(size),
        1 => exec_v1(size),
        _ => panic!("invalid version")
    };

    let total_duration = instant.elapsed().as_nanos().to_string();
    
    let solve_duration = report.duration.as_nanos().to_string();

    println!("{:},{:}", solve_duration, total_duration);
}