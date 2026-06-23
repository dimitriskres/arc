
pub trait FieldLike<Model>
where 
    Model: crate::model::ModelLike
{
    fn active(& self, atom: Model::Atom) -> bool;

    fn remove(& mut self, atom: Model::Atom);

    fn insert(& mut self, atom: Model::Atom);

    fn iter(& self, node: Model::Node) -> impl Iterator<Item = Model::Unit>;
}

#[derive(Debug, Clone)]
pub struct FieldV1<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    store: std::collections::HashSet<Model::Atom, Hasher>
}

impl<Model, Hasher> FieldV1<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, store: std::collections::HashSet<Model::Atom, Hasher>) -> Self
    {
        return Self { model, store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn store(& self) -> & std::collections::HashSet<Model::Atom, Hasher>
    {
        return & self.store;
    }
}

impl<Model, Hasher> FieldLike<Model> for FieldV1<Model, Hasher>
where 
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher
{
    fn active(& self, atom: Model::Atom) -> bool
    {
        return self.store.contains(& atom);
    }

    fn remove(& mut self, atom: Model::Atom)
    {
        self.store.remove(& atom);
    }

    fn insert(& mut self, atom: Model::Atom)
    {
        self.store.insert(atom);
    }

    fn iter(& self, node: Model::Node) -> impl Iterator<Item = Model::Unit>
    {
        let units = self.model.units(node);

        let units = units.filter(move |& unit| self.active(self.model.encode_atom(node, unit)));

        return units;
    }
}

impl<Model, Hasher> From<Model> for FieldV1<Model, Hasher>
where 
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher + Default
{
    fn from(model: Model) -> Self
    {
        let store = model.atoms().collect();

        return Self::new(model, store);
    }
}

#[derive(Debug, Clone)]
pub struct FieldV2<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    store: std::collections::HashMap<Model::Node, std::collections::HashSet<Model::Unit, Hasher>, Hasher>
}

impl<Model, Hasher> FieldV2<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, store: std::collections::HashMap<Model::Node, std::collections::HashSet<Model::Unit, Hasher>, Hasher>) -> Self
    {
        return Self { model, store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn store(& self) -> & std::collections::HashMap<Model::Node, std::collections::HashSet<Model::Unit, Hasher>, Hasher>
    {
        return & self.store;
    }
}

impl<Model, Hasher> FieldLike<Model> for FieldV2<Model, Hasher>
where 
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher
{
    fn active(& self, atom: Model::Atom) -> bool
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = self.store.get(& node).unwrap();

        return units.contains(& unit);
    }

    fn remove(& mut self, atom: Model::Atom)
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = self.store.get_mut(& node).unwrap();

        units.remove(& unit);
    }

    fn insert(& mut self, atom: Model::Atom)
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = self.store.get_mut(& node).unwrap();

        units.insert(unit);
    }

    fn iter(& self, node: Model::Node) -> impl Iterator<Item = Model::Unit>
    {
        let units = self.store.get(& node).unwrap();

        let units = units.iter().copied();

        return units;
    }
}

impl<Model, Hasher> From<Model> for FieldV2<Model, Hasher>
where 
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher + Default
{
    fn from(model: Model) -> Self
    {
        let store = model.nodes().map(|node| (node, model.units(node).collect())).collect();

        return Self::new(model, store);
    }
}

#[derive(Debug, Clone)]
pub struct FieldV3<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    store: Box<[std::collections::HashSet<Model::Unit, Hasher>]>
}

impl<Model, Hasher> FieldLike<Model> for FieldV3<Model, Hasher>
where 
    Model: crate::model::ModelLike<Node = crate::codec::Scalar, Unit: Eq + std::hash::Hash>,
    Hasher: std::hash::BuildHasher
{
    fn active(& self, atom: Model::Atom) -> bool
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = & self.store[node as usize];

        return units.contains(& unit);
    }

    fn remove(& mut self, atom: Model::Atom)
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = & mut self.store[node as usize];

        units.remove(& unit);
    }

    fn insert(& mut self, atom: Model::Atom)
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = & mut self.store[node as usize];

        units.insert(unit);
    }

    fn iter(& self, node: Model::Node) -> impl Iterator<Item = Model::Unit>
    {
        let units = & self.store[node as usize];

        let units = units.iter().copied();

        return units;
    }
}

impl<Model, Hasher> From<Model> for FieldV3<Model, Hasher>
where 
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>,
    Hasher: std::hash::BuildHasher + Default
{
    fn from(model: Model) -> Self
    {
        let store = model.nodes().map(|node| model.units(node).collect()).collect();

        return Self { model, store };
    }
}

#[derive(Debug, Clone)]
pub struct FieldV4<Model>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    store: Box<[Vec<Model::Unit>]>
}

impl<Model> FieldLike<Model> for FieldV4<Model>
where 
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>
{
    fn active(& self, atom: Model::Atom) -> bool
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = & self.store[node as usize];

        return units.contains(& unit);
    }

    fn remove(& mut self, atom: Model::Atom)
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = & mut self.store[node as usize];

        if let Some(index) = units.iter().position(|& u| u == unit)
        {
            units.swap_remove(index);
        };
    }

    fn insert(& mut self, atom: Model::Atom)
    {
        let (node, unit) = self.model.decode_atom(atom);

        let units = & mut self.store[node as usize];

        if !units.contains(& unit)
        {
            units.push(unit);
        };
    }

    fn iter(& self, node: Model::Node) -> impl Iterator<Item = Model::Unit>
    {
        let units = & self.store[node as usize];

        let units = units.iter().copied();

        return units;
    }
}

impl<Model> From<Model> for FieldV4<Model>
where 
    Model: crate::model::ModelLike<Node = crate::codec::Scalar>
{
    fn from(model: Model) -> Self
    {
        let store = model.nodes().map(|node| model.units(node).collect()).collect();

        return Self { model, store };
    }
}

#[derive(Debug, Clone)]
pub struct FieldV5<Model, BitSet>
{
    model: Model,
    store: BitSet
}

impl<Model, BitSet> FieldV5<Model, BitSet>
where 
    Model: crate::model::ModelLike<Node = crate::codec::Scalar, Unit = crate::codec::Scalar, Atom = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
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

impl<Model, BitSet> FieldLike<Model> for FieldV5<Model, BitSet>
where 
    Model: crate::model::ModelLike<Node = crate::codec::Scalar, Unit = crate::codec::Scalar, Atom = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{
    fn active(& self, atom: Model::Atom) -> bool
    {
        let atom_index = atom as usize;

        return self.store.get(atom_index);
    }

    fn remove(& mut self, atom: Model::Atom)
    {
        let atom_index = atom as usize;

        self.store.remove(atom_index);
    }

    fn insert(& mut self, atom: Model::Atom)
    {
        let atom_index = atom as usize;

        self.store.insert(atom_index);
    }

    fn iter(& self, node: Model::Node) -> impl Iterator<Item = Model::Unit>
    {
        let start = self.model.encode_atom(node, 0) as usize;

        let count = self.model.unit_count(node) as usize;

        let limit = start + count;
        
        let units = self.store.iter_with_range(start, limit).map(move |atom_index| (atom_index - start) as Model::Unit);

        return units;
    }
}

impl<Model, BitSet> From<Model> for FieldV5<Model, BitSet>
where 
    Model: crate::model::ModelLike<Node = crate::codec::Scalar, Unit = crate::codec::Scalar, Atom = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{
    fn from(model: Model) -> Self
    {
        let atom_count = model.atom_count();

        let store = BitSet::new(atom_count, true);

        return Self::new(model, store);
    }
}