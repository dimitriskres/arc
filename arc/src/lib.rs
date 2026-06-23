
pub mod codec;
pub mod model;
pub mod solver;
pub mod assert;
pub mod coerce;
pub mod analyze;

#[macro_export]
macro_rules! resolve_field 
{
    (F1R) => { arc::assert::field::FieldV1<_, std::collections::hash_map::RandomState> };
    (F1X) => { arc::assert::field::FieldV1<_, rustc_hash::FxBuildHasher> };
    (F2R) => { arc::assert::field::FieldV2<_, std::collections::hash_map::RandomState> };
    (F2X) => { arc::assert::field::FieldV2<_, rustc_hash::FxBuildHasher> };
    (F3R) => { arc::assert::field::FieldV3<_, std::collections::hash_map::RandomState> };
    (F3X) => { arc::assert::field::FieldV2<_, rustc_hash::FxBuildHasher> };
    (F4) => { arc::assert::field::FieldV4<_> };
    (F5F) => { arc::assert::field::FieldV5<_, bitset::flat::BitSet> };
    (F5M) => { arc::assert::field::FieldV5<_, bitset::meta::BitSet> };
}

#[macro_export]
macro_rules! resolve_queue 
{
    (Q1) => { arc::assert::queue::QueueV1<_> };
    (Q2R) => { arc::assert::queue::QueueV2<_, std::collections::hash_map::RandomState> };
    (Q2X) => { arc::assert::queue::QueueV2<_, rustc_hash::FxBuildHasher> };
    (Q3F) => { arc::assert::queue::QueueV3<_, bitset::flat::BitSet> };
    (Q3M) => { arc::assert::queue::QueueV3<_, bitset::meta::BitSet> };
    (Q4F) => { arc::assert::queue::QueueV4<_, bitset::flat::BitSet> };
    (Q4M) => { arc::assert::queue::QueueV4<_, bitset::meta::BitSet> };
}

#[macro_export]
macro_rules! resolve_cache
{
    (C1R) => { arc::assert::cache::CacheV1<_, std::collections::hash_map::RandomState> };
    (C1X) => { arc::assert::cache::CacheV1<_, rustc_hash::FxBuildHasher> };
    (C2R) => { arc::assert::cache::CacheV2<_, std::collections::hash_map::RandomState> };
    (C2X) => { arc::assert::cache::CacheV1<_, rustc_hash::FxBuildHasher> };
    (C3) => { arc::assert::cache::CacheV3<_> };
    (C4F) => { arc::assert::cache::CacheV4<_, bitset::flat::BitSet> };
    (C4M) => { arc::assert::cache::CacheV4<_, bitset::meta::BitSet> };
    (C5F) => { arc::assert::cache::CacheV5<_, bitset::flat::BitSet> };
    (C5M) => { arc::assert::cache::CacheV5<_, bitset::meta::BitSet> };
}

#[macro_export]
macro_rules! resolve_probe 
{
    (P1) => { arc::coerce::probe::ProbeV1<_> };
    (P2) => { arc::coerce::probe::ProbeV2<_> };
    (P3) => { arc::coerce::probe::ProbeV3<_> };
}

#[macro_export]
macro_rules! bench
{
    ($audit:path, $field:ident, $queue:ident, $cache:ident, $probe:ident, $model:expr, $minimum:expr, $prepare:expr, $timeout:expr) => 
    {
        arc::analyze::bench::<_, $audit, arc::resolve_field!($field), arc::resolve_queue!($queue), arc::resolve_cache!($cache), arc::resolve_probe!($probe)>($model, $minimum, $prepare, $timeout)
    };
}

#[macro_export]
macro_rules! bench_cases
{
    ($name:expr, $audit:path, $handle:ident, $model:ident, $minimum:expr, $prepare:expr, $timeout:expr, $([$field:ident $queue:ident $cache:ident $probe:ident]),+ $(,)?) => 
    {
        $(
            paste::paste!
            {
                let name_field = stringify!($field);
                let name_queue = stringify!($queue);
                let name_cache = stringify!($cache);
                let name_probe = stringify!($probe);

                let name = format!("{}_{}_{}_{}_{}", $name, name_field, name_queue, name_cache, name_probe);

                println!("{:}", name);

                let report = arc::bench!($audit, $field, $queue, $cache, $probe, $model, $minimum, $prepare, $timeout);

                $handle((& name, report));
            }
        )+
    };
}