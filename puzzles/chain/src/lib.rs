pub mod model;
pub mod method;

pub fn validate<Model, Field>(model: Model, field: & Field) -> String
where 
    Model: crate::model::ModelLike,
    Field: arc::assert::field::FieldLike<Model>
{
    let mut prev = None;
    
    for node in model.nodes()
    {
        let mut units = field.iter(node);

        let Some(unit) = units.next() else
        {
            return format!("node {:?} has no unit", node);
        };

        if units.next().is_some()
        {
            return format!("node {:?} has more than one unit", node);
        };

        if let Some(prev) = prev
        {
            if !(prev < unit)
            {
                return format!("node {:?} has unit {:?} !< {:?}", node, unit, prev);
            };
        };

        prev = Some(unit);
    };

    return format!("ok");
}
