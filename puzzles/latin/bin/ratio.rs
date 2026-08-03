
const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

fn main() 
{
    type Audit<Model> = latin::method::v1::audit::Audit<Model>;

    type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

    type Queue<Model> = arc::assert::queue::QueueV3<Model, bitset::meta::BitSet>;

    type Cache<Model> = arc::assert::cache::CacheV5<Model, bitset::flat::BitSet>;

    type Probe<Model> = arc::coerce::probe::ProbeV2<Model>;

    let file = std::fs::OpenOptions::new().create(true).append(true).open("kernel-ratio.csv").unwrap();

    let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(file);

    for size in 5..=50
    {
        let model = & latin::method::v1::model::ScalarModel::new(size);
        
        let Some(report) = arc::analyze::gauge::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model, TIMEOUT, Some(latin::validate)) else
        {
            continue;
        };

        if matches!(report.signal, arc::coerce::signal::Signal::Revert) 
        {
            continue;
        };

        let ratio = report.locates / report.coerces;

        println!("{:?}", [& size.to_string(), & ratio.to_string(), & report.duration.as_nanos().to_string()]);

        writer.write_record(& [& size.to_string(), & ratio.to_string(), & report.duration.as_nanos().to_string()]).unwrap();

        writer.flush().unwrap();
    };
}
