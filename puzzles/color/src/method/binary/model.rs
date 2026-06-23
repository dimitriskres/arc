
pub type Node = arc::codec::Scalar;
pub type Link = arc::codec::Scalar;
pub type Unit = arc::codec::Scalar;

pub type Fact<Atom> = arc::codec::Object<Atom, Link>;

pub trait ModelLike: arc::model::ModelLike
{
    #[allow(unused_variables)]
    fn links(& self, atom: Self::Atom) -> impl Iterator<Item = Link>;

    fn encode_fact(& self, atom: Self::Atom, link: Link) -> Self::Fact;

    fn decode_fact_to_atom(& self, fact: Self::Fact) -> Self::Atom;

    fn decode_fact_to_link(& self, fact: Self::Fact) -> Link;

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Atom, Link);

    fn decode_link(& self, node: Self::Node, link: Link) -> Self::Node;
}

pub type FactCodec<Atom> = arc::codec::ObjectCodec<Atom, Link>;

#[derive(Clone)]
pub struct Model<AtomCodec>
where 
    AtomCodec: arc::codec::CodecLike
{
    pub edges: Box<[Box<[Node]>]>,
    pub node_count: arc::codec::Scalar,
    pub unit_count: arc::codec::Scalar,
    pub atom_count: arc::codec::Scalar,
    pub fact_count: arc::codec::Scalar,
    pub atom_codec: AtomCodec,
    pub fact_codec: FactCodec<AtomCodec::I>
}

impl<AtomCodec> Model<AtomCodec>
where 
    AtomCodec: arc::codec::CodecLike
{
    pub fn build(edges: Box<[Box<[Node]>]>, unit_count: arc::codec::Scalar, atom_codec: AtomCodec) -> Self
    {
        let node_count = edges.len() as arc::codec::Scalar;

        let atom_count = node_count * unit_count;

        let fact_count = edges.iter().map(|edges| edges.len() as arc::codec::Scalar).sum();

        let fact_codec = arc::codec::ObjectCodec::new();

        return Self { edges, node_count, unit_count, atom_count, fact_count, atom_codec, fact_codec };
    }
}

pub type ObjectAtomCodec = arc::codec::ObjectCodec<Node, Unit>;

pub type ObjectModel = Model<ObjectAtomCodec>;

impl ObjectModel
{
    pub fn new(edges: Box<[Box<[Node]>]>, unit_count: arc::codec::Scalar) -> Self
    {
        let atom_codec = arc::codec::ObjectCodec::new();

        return Self::build(edges, unit_count, atom_codec);
    }
}

pub type ScalarAtomCodec = arc::codec::UniformCodec;

pub type ScalarModel = Model<ScalarAtomCodec>;

impl ScalarModel
{
    pub fn new(edges: Box<[Box<[Node]>]>, unit_count: arc::codec::Scalar) -> Self
    {
        let atom_codec = arc::codec::UniformCodec::new(unit_count);

        return Self::build(edges, unit_count, atom_codec);
    }
}

impl<AtomCodec> arc::model::ModelLike for & Model<AtomCodec>
where 
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>
{
    type Node = Node;
    type Unit = Unit;
    type Atom = AtomCodec::I;
    type Fact = Fact<AtomCodec::I>;

    fn node_count(& self) -> usize
    {
        return self.node_count as usize;
    }

    fn nodes(& self) -> impl Iterator<Item = Self::Node>
    {
        return 0..self.node_count;
    }

    #[allow(unused_variables)]
    fn unit_count(& self, node: Self::Node) -> usize
    {
        return self.unit_count as usize;
    }

    #[allow(unused_variables)]
    fn units(& self, node: Self::Node) -> impl Iterator<Item = Self::Unit>
    {
        return 0..self.unit_count;
    }

    fn atom_count(& self) -> usize
    {
        return self.atom_count as usize;
    }

    fn atom_scope(& self, atom: Self::Atom) -> impl Iterator<Item = Self::Fact>
    {
        return self.links(atom).map(move |link| self.encode_fact(atom, link));
    }

    #[allow(unused_variables)]
    fn atom_scope_size(& self, atom: Self::Atom) -> usize
    {
        return self.fact_count();
    }

    fn atoms(& self) -> impl Iterator<Item = Self::Atom>
    {
        return self.nodes().flat_map(move |node| self.units(node).map(move |unit| arc::codec::CodecLike::encode(& self.atom_codec, node, unit)));
    }

    fn encode_atom(& self, node: Self::Node, unit: Self::Unit) -> Self::Atom
    {
        return arc::codec::CodecLike::encode(& self.atom_codec, node, unit);
    }

    fn decode_atom_to_node(& self, atom: Self::Atom) -> Self::Node
    {
        return arc::codec::CodecLike::decode_to_x(& self.atom_codec, atom);
    }

    fn decode_atom_to_unit(& self, atom: Self::Atom) -> Self::Unit
    {
        return arc::codec::CodecLike::decode_to_y(& self.atom_codec, atom);
    }

    fn decode_atom(& self, atom: Self::Atom) -> (Self::Node, Self::Unit)
    {
        return arc::codec::CodecLike::decode(& self.atom_codec, atom);
    }

    fn fact_count(& self) -> usize
    {
        return self.fact_count as usize;
    }

    fn fact_scope(& self, fact: Self::Fact) -> impl Iterator<Item = Self::Atom>
    {
        return std::iter::once(self.decode_fact_to_atom(fact));
    }

    #[allow(unused_variables)]
    fn fact_scope_size(& self, fact: Self::Fact) -> usize
    {
        return 1;
    }

    fn facts(& self) -> impl Iterator<Item = Self::Fact>
    {
        return self.atoms().flat_map(move |atom| self.links(atom).map(move |link| arc::codec::CodecLike::encode(& self.fact_codec, atom, link)));
    }
}

impl<AtomCodec> ModelLike for & Model<AtomCodec>
where 
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>
{
    fn links(& self, atom: Self::Atom) -> impl Iterator<Item = Link>
    {
        let node = arc::model::ModelLike::decode_atom_to_node(self, atom);

        let node_index = node as usize;

        let edges = & self.edges[node_index];

        let link_count = edges.len() as Link;

        return 0..link_count;
    }

    fn encode_fact(& self, atom: Self::Atom, link: Link) -> Self::Fact
    {
        return arc::codec::CodecLike::encode(& self.fact_codec, atom, link);
    }

    fn decode_fact_to_atom(& self, fact: Self::Fact) -> Self::Atom
    {
        return arc::codec::CodecLike::decode_to_x(& self.fact_codec, fact);
    }

    fn decode_fact_to_link(& self, fact: Self::Fact) -> Link
    {
        return arc::codec::CodecLike::decode_to_y(& self.fact_codec, fact);
    }

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Atom, Link)
    {
        return arc::codec::CodecLike::decode(& self.fact_codec, fact);
    }

    fn decode_link(& self, node: Self::Node, link: Link) -> Self::Node 
    {
        let node_index = node as usize;

        let edges = & self.edges[node_index];

        let target_node = edges[link as usize];

        return target_node;
    }
}
