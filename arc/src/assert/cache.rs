
pub trait CacheLike<Model>
where 
    Model: crate::model::ModelLike
{
    fn active(& self, fact: Model::Fact) -> bool;

    fn insert(& mut self, fact: Model::Fact, atoms: & [Model::Atom]);

    fn remove<Emit>(& mut self, atom: Model::Atom, emit: Emit)
    where 
        Emit: FnMut(Model::Fact);
}

#[derive(Debug, Clone)]
pub struct CacheV1<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    edge_store: std::collections::HashMap<Model::Atom, Vec<Model::Fact>, Hasher>,
    fact_store: std::collections::HashSet<Model::Fact, Hasher>
}

impl<Model, Hasher> CacheV1<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, edge_store: std::collections::HashMap<Model::Atom, Vec<Model::Fact>, Hasher>, fact_store: std::collections::HashSet<Model::Fact, Hasher>) -> Self
    {
        return Self { model, edge_store, fact_store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn edge_store(& self) -> & std::collections::HashMap<Model::Atom, Vec<Model::Fact>, Hasher>
    {
        return & self.edge_store;
    }

    pub fn fact_store(& self) -> & std::collections::HashSet<Model::Fact, Hasher>
    {
        return & self.fact_store;
    }
}

impl<Model, Hasher> CacheLike<Model> for CacheV1<Model, Hasher>
where
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher
{ 
    fn active(& self, fact: Model::Fact) -> bool
    {
        return self.fact_store.contains(& fact);
    }

    fn insert(& mut self, fact: Model::Fact, atoms: & [Model::Atom])
    {
        self.fact_store.insert(fact);

        for atom in atoms.iter().copied()
        {
            self.edge_store.entry(atom).or_default().push(fact);
        };
    }

    fn remove<Emit>(& mut self, atom: Model::Atom, mut emit: Emit)
    where 
        Emit: FnMut(Model::Fact)
    {
        let edge_store = self.edge_store.entry(atom).or_default();

        for fact in edge_store.iter().copied()
        {
            self.fact_store.remove(& fact);

            emit(fact);
        };

        self.edge_store.remove(& atom);
    }
}

impl<Model, Hasher> From<Model> for CacheV1<Model, Hasher>
where 
    Model: crate::model::ModelLike,
    Hasher: std::hash::BuildHasher + Default
{
    fn from(model: Model) -> Self
    {
        let edge_store = std::collections::HashMap::default();

        let fact_store = std::collections::HashSet::default();

        return Self { model, edge_store, fact_store };
    }
}

#[derive(Debug, Clone)]
pub struct CacheV2<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    edge_store: Box<[Vec<Model::Fact>]>,
    fact_store: std::collections::HashSet<Model::Fact, Hasher>
}

impl<Model, Hasher> CacheV2<Model, Hasher>
where 
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, edge_store: Box<[Vec<Model::Fact>]>, fact_store: std::collections::HashSet<Model::Fact, Hasher>) -> Self
    {
        return Self { model, edge_store, fact_store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn edge_store(& self) -> & Box<[Vec<Model::Fact>]>
    {
        return & self.edge_store;
    }

    pub fn fact_store(& self) -> & std::collections::HashSet<Model::Fact, Hasher>
    {
        return & self.fact_store;
    }
}

impl<Model, Hasher> CacheLike<Model> for CacheV2<Model, Hasher>
where
    Model: crate::model::ModelLike<Atom = crate::codec::Scalar>,
    Hasher: std::hash::BuildHasher
{ 
    fn active(& self, fact: Model::Fact) -> bool
    {
        return self.fact_store.contains(& fact);
    }

    fn insert(& mut self, fact: Model::Fact, atoms: & [Model::Atom])
    {
        self.fact_store.insert(fact);

        for atom in atoms.iter().copied()
        {
            let atom_index = atom as usize;

            self.edge_store[atom_index].push(fact);
        };
    }

    fn remove<Emit>(& mut self, atom: Model::Atom, mut emit: Emit)
    where 
        Emit: FnMut(Model::Fact)
    {
        let atom_index = atom as usize;

        let edges = & mut self.edge_store[atom_index];

        for fact in edges.iter().copied()
        {
            self.fact_store.remove(& fact);

            emit(fact);
        };

        edges.clear();
    }
}

impl<Model, Hasher> From<Model> for CacheV2<Model, Hasher>
where 
    Model: crate::model::ModelLike<Atom = crate::codec::Scalar>,
    Hasher: std::hash::BuildHasher + Default
{
    fn from(model: Model) -> Self
    {
        let atom_count = model.atom_count();

        let edge_store = vec![vec![]; atom_count].into_boxed_slice();

        let fact_store = std::collections::HashSet::default();

        return Self { model, edge_store, fact_store };
    }
}

#[derive(Debug, Clone)]
pub struct CacheV3<Model>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    edge_store: Box<[Vec<Model::Fact>]>,
    fact_store: Box<[bool]>
}

impl<Model> CacheV3<Model>
where 
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, edge_store: Box<[Vec<Model::Fact>]>, fact_store: Box<[bool]>) -> Self
    {
        return Self { model, edge_store, fact_store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn edge_store(& self) -> & Box<[Vec<Model::Fact>]>
    {
        return & self.edge_store;
    }

    pub fn fact_store(& self) -> & Box<[bool]>
    {
        return & self.fact_store;
    }
}

impl<Model> CacheLike<Model> for CacheV3<Model>
where
    Model: crate::model::ModelLike<Atom = crate::codec::Scalar, Fact = crate::codec::Scalar>
{ 
    fn active(& self, fact: Model::Fact) -> bool
    {
        let fact_index = fact as usize;

        return self.fact_store[fact_index];
    }

    fn insert(& mut self, fact: Model::Fact, atoms: & [Model::Atom])
    {
        let fact_index = fact as usize;

        self.fact_store[fact_index] = true;

        for atom in atoms.iter().copied()
        {
            let atom_index = atom as usize;

            self.edge_store[atom_index].push(fact);
        };
    }

    fn remove<Emit>(& mut self, atom: Model::Atom, mut emit: Emit)
    where 
        Emit: FnMut(Model::Fact)
    {
        let atom_index = atom as usize;

        let edges = & mut self.edge_store[atom_index];

        for fact in edges.iter().copied()
        {
            let fact_index = fact as usize;

            self.fact_store[fact_index] = false;

            emit(fact);
        };

        edges.clear();
    }
}

impl<Model> From<Model> for CacheV3<Model>
where 
    Model: crate::model::ModelLike
{
    fn from(model: Model) -> Self
    {
        let atom_count = model.atom_count();

        let edge_store = vec![vec![]; atom_count].into_boxed_slice();

        let fact_count = model.fact_count();

        let fact_store = vec![false; fact_count].into_boxed_slice();

        return Self { model, edge_store, fact_store };
    }
}

#[derive(Debug, Clone)]
pub struct CacheV4<Model, BitSet>
where 
    Model: crate::model::ModelLike
{
    model: Model,
    edge_store: Box<[Vec<Model::Fact>]>,
    fact_store: BitSet
}

impl<Model, BitSet> CacheV4<Model, BitSet>
where 
    Model: crate::model::ModelLike,
    BitSet: bitset::like::BitSetLike
{
    pub fn new(model: Model, edge_store: Box<[Vec<Model::Fact>]>, fact_store: BitSet) -> Self
    {
        return Self { model, edge_store, fact_store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn edge_store(& self) -> & Box<[Vec<Model::Fact>]>
    {
        return & self.edge_store;
    }

    pub fn fact_store(& self) -> & BitSet
    {
        return & self.fact_store;
    }
}

impl<Model, BitSet> CacheLike<Model> for CacheV4<Model, BitSet>
where
    Model: crate::model::ModelLike<Atom = crate::codec::Scalar, Fact = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{ 
    fn active(& self, fact: Model::Fact) -> bool
    {
        let fact_index = fact as usize;

        return self.fact_store.get(fact_index);
    }

    fn insert(& mut self, fact: Model::Fact, atoms: & [Model::Atom])
    {
        let fact_index = fact as usize;

        self.fact_store.insert(fact_index);

        for atom in atoms.iter().copied()
        {
            let atom_index = atom as usize;
            
            self.edge_store[atom_index].push(fact);
        };
    }

    fn remove<Emit>(& mut self, atom: Model::Atom, mut emit: Emit)
    where 
        Emit: FnMut(Model::Fact)
    {
        let atom_index = atom as usize;

        let edges = & mut self.edge_store[atom_index];

        for fact in edges.iter().copied()
        {
            let fact_index = fact as usize;

            self.fact_store.remove(fact_index);

            emit(fact);
        };

        edges.clear();
    }
}

impl<Model, BitSet> From<Model> for CacheV4<Model, BitSet>
where 
    Model: crate::model::ModelLike<Atom = crate::codec::Scalar, Fact = crate::codec::Scalar>,
    BitSet: bitset::like::BitSetLike
{
    fn from(model: Model) -> Self
    {
        let atom_count = model.atom_count();

        let edge_store = vec![vec![]; atom_count].into_boxed_slice();

        let fact_count = model.fact_count();

        let fact_store = BitSet::new(fact_count, false);

        return Self { model, edge_store, fact_store };
    }
}

pub trait CacheBiModelLike
where 
    Self: crate::model::ModelLike
{
    fn edge_count(& self, atom: Self::Atom) -> usize;

    fn edge_start(& self, atom: Self::Atom) -> usize;

    fn encode_edge(& self, atom: Self::Atom, fact: Self::Fact) -> usize;

    fn decode_edge(& self, atom: Self::Atom, edge: usize) -> Self::Fact;
}

#[derive(Debug, Clone)]
pub struct CacheV5<Model, BitSet>
{
    model: Model,
    edge_store: BitSet,
    fact_store: BitSet
}

impl<Model, BitSet> CacheV5<Model, BitSet>
where 
    Model: crate::model::ModelLike,
    BitSet: bitset::like::BitSetLike
{
    pub fn new(model: Model, edge_store: BitSet, fact_store: BitSet) -> Self
    {
        return Self { model, edge_store, fact_store };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn edge_store(& self) -> & BitSet
    {
        return & self.edge_store;
    }

    pub fn fact_store(& self) -> & BitSet
    {
        return & self.fact_store;
    }
}

impl<Model, BitSet> CacheLike<Model> for CacheV5<Model, BitSet>
where
    Model: crate::model::ModelLike<Fact = crate::codec::Scalar> + CacheBiModelLike,
    BitSet: bitset::like::BitSetLike
{
    fn active(& self, fact: Model::Fact) -> bool
    {
        let fact_index = fact as usize;

        return self.fact_store.get(fact_index);
    }

    fn insert(& mut self, fact: Model::Fact, atoms: & [Model::Atom])
    {
        let fact_index = fact as usize;

        self.fact_store.insert(fact_index);

        for atom in atoms.iter().copied()
        {
            let edge = self.model.encode_edge(atom, fact);

            let edge_index = self.model.edge_start(atom) + edge;

            self.edge_store.insert(edge_index);
        };
    }

    fn remove<Emit>(& mut self, atom: Model::Atom, mut emit: Emit)
    where 
        Emit: FnMut(Model::Fact)
    {
        let edge_start = self.model.edge_start(atom);

        let edge_count = self.model.edge_count(atom);

        let edge_limit = edge_start + edge_count;

        for edge_index in self.edge_store.iter_with_range(edge_start, edge_limit)
        {
            let edge = edge_index - edge_start;

            let fact = self.model.decode_edge(atom, edge);

            let fact_index = fact as usize;

            self.fact_store.remove(fact_index);

            emit(fact);
        };

        self.edge_store.drop_with_range(edge_start, edge_limit);
    }
}

impl<Model, BitSet> From<Model> for CacheV5<Model, BitSet>
where 
    Model: crate::model::ModelLike + CacheBiModelLike,
    BitSet: bitset::like::BitSetLike
{
    fn from(model: Model) -> Self
    {
        let atoms = model.atoms();

        let edge_count = atoms.map(|atom| model.edge_count(atom)).sum();

        let edge_store = BitSet::new(edge_count, false);

        let fact_count = model.fact_count();

        let fact_store = BitSet::new(fact_count, false);

        return Self { model, edge_store, fact_store };
    }
}