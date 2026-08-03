
pub type Node = arc::codec::Scalar;
pub type Unit = arc::codec::Scalar;
pub type Link = arc::codec::Scalar;

pub trait ModelLike: arc::model::ModelLike
{
    fn edges(& self, node: Self::Node) -> & [Self::Node];

    fn links(& self, node: Self::Node) -> impl Iterator<Item = Link>
    {
        return 0..self.edges(node).len() as Link;
    }

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Node, Link);

    fn decode_link(& self, node: Self::Node, link: Link) -> Option<Self::Node>;
}

#[derive(Debug, Clone)]
pub struct Model<AtomCodec, FactCodec>
{
    pub edges: Box<[Box<[Node]>]>,
    pub node_count: arc::codec::Scalar,
    pub unit_count: arc::codec::Scalar,
    pub atom_count: arc::codec::Scalar,
    pub fact_count: arc::codec::Scalar,
    pub atom_codec: AtomCodec,
    pub fact_codec: FactCodec
}

impl<AtomCodec, FactCodec> arc::model::ModelLike for & Model<AtomCodec, FactCodec>
where
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = Node, Y = Link, I: arc::model::Tiny>
{
    type Node = Node;
    type Unit = Unit;
    type Atom = AtomCodec::I;
    type Fact = FactCodec::I;

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

    fn atom_scope_size(& self, atom: Self::Atom) -> usize
    {
        let node = self.decode_atom_to_node(atom);

        return self.edges(node).len();
    }

    fn atom_scope(& self, atom: Self::Atom) -> impl Iterator<Item = Self::Fact>
    {
        let node = self.decode_atom_to_node(atom);

        return (0..self.edges(node).len() as Link).map(move |link| arc::codec::CodecLike::encode(& self.fact_codec, node, link));
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

    #[allow(unused_variables)]
    fn fact_scope_size(& self, fact: Self::Fact) -> usize
    {
        return 0;
    }

    #[allow(unused_variables)]
    fn fact_scope(& self, fact: Self::Fact) -> impl Iterator<Item = Self::Atom>
    {
        return std::iter::empty();
    }

    fn facts(& self) -> impl Iterator<Item = Self::Fact>
    {
        return self.nodes().flat_map(move |node| self.links(node).filter(move |& link| node < self.edges(node)[link as usize]).map(move |link| arc::codec::CodecLike::encode(& self.fact_codec, node, link)));
    }
}

impl<AtomCodec, FactCodec> crate::model::ModelLike for & Model<AtomCodec, FactCodec>
where
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = Node, Y = Link, I: arc::model::Tiny>
{
    fn edges(& self, node: Self::Node) -> & [Self::Node]
    {
        return & self.edges[node as usize];
    }
}

impl<AtomCodec, FactCodec> ModelLike for & Model<AtomCodec, FactCodec>
where
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = Node, Y = Link, I: arc::model::Tiny>
{
    fn edges(& self, node: Self::Node) -> & [Self::Node]
    {
        return & self.edges[node as usize];
    }

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Node, Link)
    {
        return arc::codec::CodecLike::decode(& self.fact_codec, fact);
    }

    fn decode_link(& self, node: Self::Node, link: Link) -> Option<Node>
    {
        let edges = self.edges(node);

        let node = edges.get(link as usize).copied();

        return node;
    }
}

pub type ObjectAtom = arc::codec::Object<Node, Unit>;
pub type ObjectFact = arc::codec::Object<Node, Link>;

pub type ObjectAtomCodec = arc::codec::ObjectCodec<Node, Unit>;
pub type ObjectFactCodec = arc::codec::ObjectCodec<Node, Link>;

pub type ObjectModel = Model<ObjectAtomCodec, ObjectFactCodec>;

impl ObjectModel
{
    pub fn new(edges: Box<[Box<[Node]>]>, unit_count: arc::codec::Scalar) -> Self
    {
        let node_count = edges.len() as arc::codec::Scalar;

        let atom_count = node_count * unit_count;

        let fact_count = edges.iter().map(|edge| edge.len()).sum::<usize>() as arc::codec::Scalar;

        let atom_codec = arc::codec::ObjectCodec::new();

        let fact_codec = arc::codec::ObjectCodec::new();

        return Self { edges, node_count, unit_count, atom_count, fact_count, atom_codec, fact_codec };
    }
}

pub type ScalarAtom = arc::codec::Scalar;
pub type ScalarFact = arc::codec::Scalar;

pub type ScalarAtomCodec = arc::codec::ScalarCodec;
pub type ScalarFactCodec = arc::codec::ScalarCodec;

pub type ScalarModel = Model<ScalarAtomCodec, ScalarFactCodec>;

impl ScalarModel
{
    pub fn new(edges: Box<[Box<[Node]>]>, unit_count: arc::codec::Scalar) -> Self
    {
        let node_count = edges.len() as arc::codec::Scalar;

        let atom_count = node_count * unit_count;

        let link_count = edges.iter().map(|edge| edge.len()).max().unwrap() as arc::codec::Scalar;

        let fact_count = node_count * link_count;

        let atom_codec = arc::codec::ScalarCodec::new(unit_count);

        let fact_codec = arc::codec::ScalarCodec::new(link_count);

        return Self { edges, node_count, unit_count, atom_count, fact_count, atom_codec, fact_codec };
    }
}