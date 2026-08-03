
pub trait ProbeLike<Model, Field>
where 
    Model: crate::model::ModelLike
{
    fn select(& mut self, field: & Field, node: & mut Option<Model::Node>, units: & mut Vec<Model::Unit>);

    fn remove_many(& mut self, node: Model::Node, units: & [Model::Unit]);

    fn remove(& mut self, node: Model::Node, unit: Model::Unit)
    {
        self.remove_many(node, & [unit]);
    }

    fn insert_many(& mut self, node: Model::Node, units: & [Model::Unit]);
    
    fn insert(& mut self, node: Model::Node, unit: Model::Unit)
    {
        self.insert_many(node, & [unit]);
    }
}

#[derive(Debug, Clone)]
pub struct TrackV1<Model>
where
    Model: crate::model::ModelLike
{
    sizes: Vec<(Model::Node, usize)>,
    steps: Vec<usize>
}

impl <Model> TrackV1<Model>
where
    Model: crate::model::ModelLike
{
    pub fn new() -> Self
    {
        let sizes = Vec::new();

        let steps = Vec::new();

        return Self { sizes, steps };
    }
}

#[derive(Debug, Clone)]
pub struct ProbeV1<Model>
where
    Model: crate::model::ModelLike
{
    model: Model,
    sizes: Box<[usize]>,
    track: TrackV1<Model>
}

impl<Model> ProbeV1<Model>
where
    Model: crate::model::ModelLike
{
    pub fn new(model: Model) -> Self
    {
        let sizes = model.nodes().map(|node| model.unit_count(node)).collect();

        let track = TrackV1::new();

        return Self { model, sizes, track };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn sizes(& self) -> & [usize]
    {
        return & self.sizes;
    }
}

impl<Model, Field> ProbeLike<Model, Field> for ProbeV1<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar, Unit = crate::codec::Scalar>,
    Field: crate::assert::field::FieldLike<Model>,
{
    fn select(& mut self, field: & Field, coerce_node: & mut Option<Model::Node>, coerce_units: & mut Vec<Model::Unit>)
    {
        let mut result = None;

        for (node, size) in self.sizes.iter().copied().enumerate()
        {
            if size <= 1
            {
                continue;
            };

            let replace = match result
            {
                None => true,
                Some((_, best_size)) => size < best_size
            };

            if replace
            {
                result = Some((node, size));

                if size == 2
                {
                    break;
                };
            };
        };

        let Some((node, _)) = result else
        {
            return;
        };

        let node = node as Model::Node;

        let unit = if Field::ORDERED { field.iter(node).next() } else { field.iter(node).min() };

        let unit = unit.unwrap();

        coerce_units.push(unit);

        * coerce_node = Some(node);
    }

    fn insert_many(& mut self, node: Model::Node, units: & [Model::Unit])
    {
        self.sizes[node as usize] += units.len();
    }

    fn remove_many(& mut self, node: Model::Node, units: & [Model::Unit])
    {
        let size = units.len();

        self.track.sizes.push((node, size));

        self.sizes[node as usize] -= size;
    }
}

impl<Model> From<Model> for ProbeV1<Model>
where
    Model: crate::model::ModelLike
{
    fn from(model: Model) -> Self
    {
        return Self::new(model);
    }
}

impl<Model> crate::coerce::revert::Revertible for ProbeV1<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>
{
    fn save(& mut self)
    {
        self.track.steps.push(self.track.sizes.len());
    }

    fn load(& mut self) -> bool
    {
        let Some(index) = self.track.steps.pop() else
        {
            return false;
        };

        for (node, size) in self.track.sizes[index..].iter().copied()
        {
            let node_index = node as usize;

            self.sizes[node_index] += size;
        };

        self.track.sizes.truncate(index);

        return true;
    }
}

#[derive(Debug, Clone)]
pub struct ProbeV2<Model>
where
    Model: crate::model::ModelLike
{
    model: Model,
    sizes: Box<[usize]>,
    track: TrackV1<Model>
}

impl<Model> ProbeV2<Model>
where
    Model: crate::model::ModelLike
{
    pub fn new(model: Model) -> Self
    {
        let sizes = model.nodes().map(|node| model.unit_count(node)).collect();

        let track = TrackV1::new();

        return Self { model, sizes, track };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn sizes(& self) -> & [usize]
    {
        return & self.sizes;
    }
}

fn fill_bisective_unordered<Model, Field>(field: & Field, node: Model::Node, units: & mut Vec<Model::Unit>)
where
    Model: crate::model::ModelLike<Unit = crate::codec::Scalar>,
    Field: crate::assert::field::FieldLike<Model>,
{
    units.extend(field.iter(node));

    let min = units.iter().copied().min().unwrap();
    let max = units.iter().copied().max().unwrap();
    
    let pivot = min + (max - min) / 2;

    let mut slice = 0;

    for unit_index in 0..units.len()
    {
        let unit = units[unit_index];

        if unit > pivot
        {
            continue;
        };

        units[slice] = unit;

        slice += 1;
    };

    units.truncate(slice);
}

fn fill_bisective_ordered<Model, Field>(field: & Field, node: Model::Node, units: & mut Vec<Model::Unit>)
where
    Model: crate::model::ModelLike<Unit = crate::codec::Scalar>,
    Field: crate::assert::field::FieldLike<Model>,
{
    units.extend(field.iter(node));

    let min = units[0];
    let max = units[units.len() - 1];
    
    let pivot = min + (max - min) / 2;

    let slice = units.partition_point(|& unit| unit <= pivot);

    units.truncate(slice);
}

fn fill_bisective<Model, Field>(field: & Field, node: Model::Node, units: & mut Vec<Model::Unit>)
where
    Model: crate::model::ModelLike<Unit = crate::codec::Scalar>,
    Field: crate::assert::field::FieldLike<Model>,
{
    if Field::ORDERED
    {
        fill_bisective_ordered::<Model, Field>(field, node, units);
    }
    else
    {
        fill_bisective_unordered::<Model, Field>(field, node, units);
    };
}

impl<Model, Field> ProbeLike<Model, Field> for ProbeV2<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar, Unit = crate::codec::Scalar>,
    Field: crate::assert::field::FieldLike<Model>,
{
    fn select(& mut self, field: & Field, coerce_node: & mut Option<Model::Node>, coerce_units: & mut Vec<Model::Unit>)
    {
        let mut result = None;

        for (node, size) in self.sizes.iter().copied().enumerate()
        {
            if size <= 1
            {
                continue;
            };

            let replace = match result
            {
                None => true,
                Some((_, best_size)) => size < best_size
            };

            if replace
            {
                result = Some((node, size));
            };
        };

        let Some((node, _)) = result else
        {
            return;
        };

        let node = node as Model::Node;

        fill_bisective(field, node, coerce_units);

        * coerce_node = Some(node);
    }

    fn insert_many(& mut self, node: Model::Node, units: & [Model::Unit])
    {
        self.sizes[node as usize] += units.len();
    }

    fn remove_many(& mut self, node: Model::Node, units: & [Model::Unit])
    {
        let size = units.len();

        self.track.sizes.push((node, size));

        self.sizes[node as usize] -= size;
    }
}

impl<Model> From<Model> for ProbeV2<Model>
where
    Model: crate::model::ModelLike
{
    fn from(model: Model) -> Self
    {
        return Self::new(model);
    }
}

impl<Model> crate::coerce::revert::Revertible for ProbeV2<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>
{
    fn save(& mut self)
    {
        self.track.steps.push(self.track.sizes.len());
    }

    fn load(& mut self) -> bool
    {
        let Some(index) = self.track.steps.pop() else
        {
            return false;
        };

        for (node, size) in self.track.sizes[index..].iter().copied()
        {
            let node_index = node as usize;

            self.sizes[node_index] += size;
        };

        self.track.sizes.truncate(index);

        return true;
    }
}
