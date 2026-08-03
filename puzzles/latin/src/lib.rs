pub mod model;
pub mod method;

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

		// latin square doesn't have edges

		let (this_row, this_col) = model.decode_node(node);

		for some_row in 0..model.size()
		{
			if some_row == this_row
			{
				continue;
			};

			let target_node = model.encode_node(some_row, this_col);

			for unit in field.iter(target_node)
			{
				if origin_unit == unit
				{
					return format!("node {:?} has unit {:?} equal in {:?}", node, origin_unit, target_node);
				};
			}; 
		};

		for some_col in 0..model.size()
		{
			if some_col == this_col
			{
				continue;
			};

			let target_node = model.encode_node(this_row, some_col);

			for unit in field.iter(target_node)
			{
				if origin_unit == unit
				{
					return format!("node {:?} has unit {:?} equal in {:?}", node, origin_unit, target_node);
				};
			}; 
		};
	};

	return format!("ok");
}