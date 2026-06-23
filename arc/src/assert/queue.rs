
pub trait QueueLike<Model>
where 
    Model: crate::model::ModelLike
{
    fn get(& mut self) -> Option<Model::Fact>;

    fn put(& mut self, fact: Model::Fact);
}

#[derive(Debug, Clone)]
pub struct QueueV1<Model>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    store: Vec<Model::Fact>
}

impl<Model> QueueV1<Model>
where 
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, store: Vec<Model::Fact>) -> Self
    {
        return Self { model, store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn store(& self) -> & Vec<Model::Fact>
    {
        return & self.store;
    }
}

impl<Model> QueueLike<Model> for QueueV1<Model>
where 
    Model: crate::model::ModelLike
{
    fn get(& mut self) -> Option<Model::Fact>
    {
        return self.store.pop();
    }

    fn put(& mut self, fact: Model::Fact)
    {
        self.store.push(fact);
    }
}

impl<Model> From<Model> for QueueV1<Model>
where 
    Model: crate::model::ModelLike
{
    fn from(model: Model) -> Self
    {
        let store = model.facts().collect();

        return Self::new(model, store);
    }
}

#[derive(Debug, Clone)]
pub struct QueueV2<Model, Hasher>
where
    Model: crate::model::ModelLike
{
    model: Model,
    store: Vec<Model::Fact>,
    dedup: std::collections::HashSet<Model::Fact, Hasher>
}

impl<Model, Hasher> QueueV2<Model, Hasher>
where
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, store: Vec<Model::Fact>, dedup: std::collections::HashSet<Model::Fact, Hasher>) -> Self
    {
        return Self { model, store, dedup };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn store(& self) -> & Vec<Model::Fact>
    {
        return & self.store;
    }

    pub fn dedup(& self) -> & std::collections::HashSet<Model::Fact, Hasher>
    {
        return & self.dedup;
    }
}

impl<Model, Hasher> QueueLike<Model> for QueueV2<Model, Hasher>
where
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher
{
    fn get(& mut self) -> Option<Model::Fact>
    {
        let fact = self.store.pop()?;

        self.dedup.remove(& fact);

        return Some(fact);
    }

    fn put(& mut self, fact: Model::Fact)
    {
        if self.dedup.contains(& fact)
        {
            return;
        };

        self.store.push(fact);
        
        self.dedup.insert(fact);
    }
}

impl<Model, Hasher> From<Model> for QueueV2<Model, Hasher>
where
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher + Default
{
    fn from(model: Model) -> Self
    {
        let store = model.facts().collect();

        let dedup = model.facts().collect();

        return Self::new(model, store, dedup);
    }
}

#[derive(Debug, Clone)]
pub struct QueueV3<Model, BitSet>
{
    model: Model,
    store: BitSet
}

impl<Model, BitSet> QueueV3<Model, BitSet>
where
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, store: BitSet) -> Self
    {
        return Self { model, store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn store(& self) -> & BitSet
    {
        return & self.store;
    }
}

impl<Model, BitSet> QueueLike<Model> for QueueV3<Model, BitSet>
where
    Model: crate::model::ModelLike<Fact = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{
    fn get(& mut self) -> Option<Model::Fact>
    {
        let fact_index = self.store.next()?;

        self.store.remove(fact_index);

        let fact = fact_index as Model::Fact;

        return Some(fact);
    }

    fn put(& mut self, fact: Model::Fact)
    {
        let fact_index = fact as usize;

        self.store.insert(fact_index);
    }
}

impl<Model, BitSet> From<Model> for QueueV3<Model, BitSet>
where
    Model: crate::model::ModelLike<Fact = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{
    fn from(model: Model) -> Self
    {
        let fact_count = model.fact_count();

        let store = bitset::like::BitSetLike::new(fact_count, true);

        return Self::new(model, store);
    }
}

#[derive(Debug, Clone)]
pub struct QueueV4<Model, BitSet>
where
    Model: crate::model::ModelLike
{
    model: Model,
    store: Vec<Model::Fact>,
    dedup: BitSet
}

impl<Model, BitSet> QueueV4<Model, BitSet>
where
    Model: crate::model::ModelLike,
    BitSet: bitset::like::BitSetLike
{
    pub fn new(model: Model, store: Vec<Model::Fact>, dedup: BitSet) -> Self
    {
        return Self { model, store, dedup };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn store(& self) -> & Vec<Model::Fact>
    {
        return & self.store;
    }

    pub fn dedup(& self) -> & BitSet
    {
        return & self.dedup;
    }
}

impl<Model, BitSet> QueueLike<Model> for QueueV4<Model, BitSet>
where
    Model: crate::model::ModelLike<Fact = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{
    fn get(& mut self) -> Option<Model::Fact>
    {
        let fact = self.store.pop()?;

        let fact_index = fact as usize;

        self.dedup.remove(fact_index);

        return Some(fact);
    }

    fn put(& mut self, fact: Model::Fact)
    {
        let fact_index = fact as usize;

        if self.dedup.get(fact_index)
        {
            return;
        };

        self.store.push(fact);
        
        self.dedup.insert(fact_index);
    }
}

impl<Model, BitSet> From<Model> for QueueV4<Model, BitSet>
where
    Model: crate::model::ModelLike<Fact = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{
    fn from(model: Model) -> Self
    {
        let fact_count = model.fact_count();

        let store = model.facts().collect();

        let dedup = bitset::like::BitSetLike::new(fact_count, true);

        return Self::new(model, store, dedup);
    }
}
