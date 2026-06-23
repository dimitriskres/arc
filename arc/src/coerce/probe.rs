
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
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
{
    sizes: Vec<(Model::Node, usize)>,
    steps: Vec<usize>
}

impl <Model> TrackV1<Model>
where
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
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
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
{
    model: Model,
    sizes: Box<[usize]>,
    track: TrackV1<Model>
}

impl<Model> ProbeV1<Model>
where
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
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
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>,
    Field: crate::assert::field::FieldLike<Model>
{
    fn select(& mut self, field: & Field, coerce_node: & mut Option<Model::Node>, coerce_units: & mut Vec<Model::Unit>)
    {
        let Some((node, size)) = self.sizes.iter().copied().enumerate().find(|item| item.1 > 1) else
        {
            return 
        };

        let kill = (size + 1) / 2;

        let node = node as Model::Node;
        
        coerce_units.extend(field.iter(node).take(kill));

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
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy + Ord,
{
    fn from(model: Model) -> Self
    {
        Self::new(model)
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
            self.sizes[node as usize] += size;
        };

        self.track.sizes.truncate(index);

        return true;
    }
}

#[derive(Debug, Clone)]
pub struct ProbeV2<Model>
where
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
{
    model: Model,
    sizes: Box<[usize]>,
    buckets: Vec<Vec<Model::Node>>,
    min: usize,
    max: usize,
    track: TrackV1<Model>
}

impl<Model> ProbeV2<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>
{
    pub fn new(model: Model) -> Self
    {
        let sizes: Box<[usize]> = model.nodes().map(|node| model.unit_count(node)).collect();

        let max = sizes.iter().copied().max().unwrap_or(0);

        let mut buckets = vec![Vec::new(); max + 1];

        for node in model.nodes()
        {
            let size = sizes[node as usize];

            if size > 1
            {
                buckets[size].push(node);
            };
        };

        let min = 2;

        let track = TrackV1::new();

        return Self { model, sizes, buckets, min, max, track };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn sizes(& self) -> & [usize]
    {
        return & self.sizes;
    }

    fn set_size(& mut self, node: Model::Node, size: usize)
    {
        self.sizes[node as usize] = size;

        if size <= 1
        {
            return;
        };

        if size > self.max
        {
            self.buckets.resize_with(size + 1, Vec::new);

            self.max = size;
        };

        self.buckets[size].push(node);

        if size < self.min
        {
            self.min = size;
        };
    }
}

impl<Model, Field> ProbeLike<Model, Field> for ProbeV2<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>,
    Field: crate::assert::field::FieldLike<Model>,
{
    fn select(& mut self, field: & Field, coerce_node: & mut Option<Model::Node>, coerce_units: & mut Vec<Model::Unit>)
    {
        while self.min <= self.max
        {
            while let Some(node) = self.buckets[self.min].last().copied()
            {
                let size = self.sizes[node as usize];

                if size != self.min
                {
                    self.buckets[self.min].pop();
                };

                let kill = (size + 1) / 2;

                coerce_units.extend(field.iter(node).take(kill));

                * coerce_node = Some(node);

                return; 
            };

            self.min += 1;
        };
    }

    fn insert_many(& mut self, node: Model::Node, units: & [Model::Unit])
    {
        let size = self.sizes[node as usize] + units.len();

        self.set_size(node, size);
    }

    fn remove_many(& mut self, node: Model::Node, units: & [Model::Unit])
    {
        let size = units.len();

        self.track.sizes.push((node, size));

        let size = self.sizes[node as usize] - size;

        self.set_size(node, size);
    }
}

impl<Model> From<Model> for ProbeV2<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>,
{
    fn from(model: Model) -> Self
    {
        return Self::new(model);
    }
}

impl<Model> crate::coerce::revert::Revertible for ProbeV2<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>,
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

        let mut cursor = index;

        while cursor < self.track.sizes.len()
        {
            let (node, size) = self.track.sizes[cursor];

            let size = self.sizes[node as usize] + size;

            self.set_size(node, size);

            cursor += 1;
        };

        self.track.sizes.truncate(index);

        return true;
    }
}

#[derive(Debug, Clone)]
pub struct ProbeV3<Model>
where
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
{
    model: Model,
    sizes: Box<[usize]>,
    track: TrackV1<Model>
}

impl<Model> ProbeV3<Model>
where
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
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

impl<Model, Field> ProbeLike<Model, Field> for ProbeV3<Model>
where
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>,
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

        coerce_units.extend(field.iter(node));
        
        let kill = (coerce_units.len() + 1) / 2;

        coerce_units.truncate(kill);

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

impl<Model> From<Model> for ProbeV3<Model>
where
    Model: crate::model::ModelLike,
    Model::Node: Eq + std::hash::Hash + Copy,
{
    fn from(model: Model) -> Self
    {
        return Self::new(model);
    }
}

impl<Model> crate::coerce::revert::Revertible for ProbeV3<Model>
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