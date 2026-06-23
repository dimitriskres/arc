pub mod method;
pub mod utility;

#[cfg(test)]
mod test
{
    type Audit<Model> = crate::method::binary::audit::Audit<Model>;

    type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

    type Queue<Model> = arc::assert::queue::QueueV2<Model>;

    type Cache<Model> = arc::assert::cache::CacheV2<Model>;

    type Probe<Model> = arc::coerce::probe::ProbeV3<Model>;

    #[test]
    fn main() 
    {
        let seed = 0;

        let rng = & mut <rand::rngs::Xoshiro256PlusPlus as rand::SeedableRng>::seed_from_u64(seed);

        let instant = std::time::Instant::now();

        let timeout = std::time::Duration::from_secs(3);

        let skip = [];

        for size in 500..=1000
        {
            if skip.contains(& size)
            {
                println!("{} -> skip", size);

                continue;
            };

            let edges = crate::utility::get_edges(rng, size, 0.5);

            for unit_count in (size / 9)..(size / 4)
            {
                let unit_count = unit_count + 4;

                let model = & crate::method::binary::model::ScalarModel::new(edges.clone(), unit_count);

                let Some(package) = arc::analyze::gauge::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model, timeout) else
                {
                    println!("{} @ {} -> timeout", size, unit_count);

                    continue;
                };

                let ratio = (package.settles + package.negates) as f64 / package.reverts as f64;

                println!("{} @ {} -> {:?} -> {:.0}", size, unit_count, package, ratio);
            };
        };

        println!("total elapsed: {:?}", instant.elapsed());
    }
}