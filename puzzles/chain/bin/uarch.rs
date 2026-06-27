
const SIZE: arc::codec::Scalar = 600;

// scalar F5F Q3M C3  P3

type Audit<Model> = chain::method::binary::audit::Audit<Model>;

type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

type Queue<Model> = arc::assert::queue::QueueV3<Model, bitset::meta::BitSet>;

type Cache<Model> = arc::assert::cache::CacheV3<Model>;

type Probe<Model> = arc::coerce::probe::ProbeV3<Model>;

fn main()
{
    let node_count = SIZE;
    let unit_count = SIZE;

    let model = & chain::method::binary::model::ScalarModel::new(node_count, unit_count);

    for _ in 0..20
    {
        let mut solver = arc::coerce::solver::Solver::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>::from(model);
        
        let mut solution = arc::solver::SolverLike::solution(& mut solver);
        
        let signal = solution.finish();
        
        std::hint::black_box(signal);
    };
}