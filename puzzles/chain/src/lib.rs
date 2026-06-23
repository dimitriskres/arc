pub mod method;

#[cfg(test)]
mod test
{
    #[test]
    fn main()
    {
        type Audit<Model> = crate::method::binary::audit::Audit<Model>;

        type Field<Model> = arc::assert::field::FieldV1<Model, std::hash::RandomState>;

        type Queue<Model> = arc::assert::queue::QueueV1<Model>;

        type Cache<Model> = arc::assert::cache::CacheV1<Model, std::hash::RandomState>;

        type Probe<Model> = arc::coerce::probe::ProbeV3<Model>;

        let model = & crate::method::binary::model::ScalarModel::new(600, 600);

        let report = arc::analyze::solve::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model);

        println!("{:?}", report);
    }
}