
const SIZE: arc::codec::Scalar = 30;

// scalar F5F Q1  C4F P3

type Audit<Model> = latin::method::binary::audit::Audit<Model>;

type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

type Queue<Model> = arc::assert::queue::QueueV1<Model>;

type Cache<Model> = arc::assert::cache::CacheV4<Model, bitset::flat::BitSet>;

type Probe<Model> = arc::coerce::probe::ProbeV3<Model>;

fn main()
{
    let size = SIZE;

    let model = & latin::method::binary::model::ScalarModel::new(size);

    for _ in 0..20
    {
        let mut solver = arc::coerce::solver::Solver::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>::from(model);
        
        let mut solution = arc::solver::SolverLike::solution(& mut solver);
        
        let signal = solution.finish();
        
        std::hint::black_box(signal);
    };
}