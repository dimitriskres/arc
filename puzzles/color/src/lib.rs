pub mod model;
pub mod method;
pub mod utility;

pub fn validate<Model, Field>(model: Model, field: & Field) -> String
where 
	Model: crate::model::ModelLike,
	Field: arc::assert::field::FieldLike<Model>
{
	for node in model.nodes()
	{
		let mut units = field.iter(node);

		let Some(origin_unit) = units.next() else
		{
			return format!("node {:?} has no unit", node);
		};

		if units.next().is_some()
        {
            return format!("node {:?} has more than one unit", node);
        };

		for edge in model.edges(node).iter().copied()
		{
			for target_unit in field.iter(edge)
			{
				if origin_unit == target_unit
				{
					return format!("node {:?} has unit {:?} equal in {:?}", node, origin_unit, edge);
				};
			};
		};
	};

	return format!("ok");
}