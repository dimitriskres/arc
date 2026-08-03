
const GRID_SEED: u64 = 0;
const GRID_SIZE: arc::codec::Scalar = 100;
const GRID_DENSITY: f64 = 0.5;

const COLOR_COUNT: arc::codec::Scalar = 200;

type Probe<Model> = arc::coerce::probe::ProbeV2<Model>;

fn exec_v0(edges: Box<[Box<[arc::codec::Scalar]>]>, unit_count: arc::codec::Scalar) -> arc::analyze::SolveReport
{
    type Audit<Model> = color::method::v0::audit::Audit<Model>;

    type Field<Model> = arc::assert::field::FieldV4<Model>;

    type Queue<Model> = arc::assert::queue::QueueV1<Model>;

    type Cache<Model> = arc::assert::cache::CacheV4<Model, bitset::flat::BitSet>;

    let model = & color::method::v0::model::ScalarModel::new(edges.clone(), unit_count);

    let report = arc::analyze::solve::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model);

    return report;
}

fn exec_v1(edges: Box<[Box<[arc::codec::Scalar]>]>, unit_count: arc::codec::Scalar) -> arc::analyze::SolveReport
{
    type Audit<Model> = color::method::v1::audit::Audit<Model>;

    type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

    type Queue<Model> = arc::assert::queue::QueueV3<Model, bitset::meta::BitSet>;

    type Cache<Model> = arc::assert::cache::CacheV3<Model>;

    let model = & color::method::v1::model::ScalarModel::new(edges.clone(), unit_count);

    let report = arc::analyze::solve::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model);

    return report;
}

#[derive(clap::Parser)]
struct Args
{
    #[arg(long)]
    version: usize,
    #[arg(long)]
    tick: arc::codec::Scalar
}

fn main()
{
    let args = <Args as clap::Parser>::parse();

    let rng_state = GRID_SEED;

    let rng = & mut <rand::rngs::Xoshiro256PlusPlus as rand::SeedableRng>::seed_from_u64(rng_state);
    
    let edges_size = GRID_SIZE * args.tick;
    
    let edges_density = GRID_DENSITY;

    let edges = color::utility::get_edges(rng, edges_size, edges_density);

    let unit_count = COLOR_COUNT;

    let instant = std::time::Instant::now();

    let report = match args.version
    {
        0 => exec_v0(edges, unit_count),
        1 => exec_v1(edges, unit_count),
        _ => panic!("invalid version")
    };

    let total_duration = instant.elapsed().as_nanos().to_string();
    
    let solve_duration = report.duration.as_nanos().to_string();

    println!("{:},{:}", solve_duration, total_duration);
}