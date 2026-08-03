
const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

type Audit<Model> = color::method::v0::audit::Audit<Model>;

type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

type Queue<Model> = arc::assert::queue::QueueV1<Model>;

type Cache<Model> = arc::assert::cache::CacheV4<Model, bitset::flat::BitSet>;

type Probe<Model> = arc::coerce::probe::ProbeV2<Model>;

fn main() 
{
    let file = std::fs::OpenOptions::new().create(true).append(true).open("kernel-ratio.csv").unwrap();

    let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(file);

    let rng = & mut <rand::rngs::Xoshiro256PlusPlus as rand::SeedableRng>::seed_from_u64(0);

    for size in (100..=2000).step_by(100)
    {
        for density in (10..=50).step_by(10).map(|x| x as f64 / 100.0)
        {
            let color_count_start = if size < 300 { 50 } else if size < 600 { 100 } else { 150 };

            for color_count in (color_count_start..=200).step_by(25)
            {
                let edges = color::utility::get_edges(rng, size, density);

                let model = & color::method::v0::model::ScalarModel::new(edges, color_count);

                println!("? {size:} {density:} {color_count:}");

                let Some(report) = arc::analyze::gauge::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model, TIMEOUT, Some(color::validate)) else
                {
                    continue;
                };

                if matches!(report.signal, arc::coerce::signal::Signal::Revert) 
                {
                    continue;
                };

                let ratio = report.locates / report.coerces;

                writer.write_record(& [& size.to_string(), & density.to_string(), & color_count.to_string(), & ratio.to_string(), & report.duration.as_nanos().to_string()]).unwrap();

                writer.flush().unwrap();
            };
        };
    };
}
