
pub type Node = arc::codec::Scalar;
pub type Link = arc::codec::Scalar;
pub type Unit = arc::codec::Scalar;

pub type Fact<Atom> = arc::codec::Object<Atom, Link>;

pub trait ModelLike: crate::model::ModelLike
{
    #[allow(unused_variables)]
    fn links(& self, atom: Self::Atom) -> impl Iterator<Item = Link>;

    fn encode_fact(& self, atom: Self::Atom, link: Link) -> Self::Fact;

    fn decode_fact_to_atom(& self, fact: Self::Fact) -> Self::Atom;

    fn decode_fact_to_link(& self, fact: Self::Fact) -> Link;

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Atom, Link);

    fn decode_link(& self, node: Self::Node, link: Link) -> Option<Self::Node>;
}

#[derive(Clone)]
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

pub type ObjectAtom = arc::codec::Object<Node, Unit>;
pub type ObjectFact = arc::codec::Object<ObjectAtom, Link>;

pub type ObjectAtomCodec = arc::codec::ObjectCodec<Node, Unit>;
pub type ObjectFactCodec = arc::codec::ObjectCodec<ObjectAtom, Link>;

pub type ObjectModel = Model<ObjectAtomCodec, ObjectFactCodec>;

impl ObjectModel
{
    pub fn new(edges: Box<[Box<[Node]>]>, unit_count: arc::codec::Scalar) -> Self
    {
        let node_count = edges.len() as arc::codec::Scalar;

        let atom_count = node_count * unit_count;

        let fact_count = unit_count * edges.iter().map(|edges| edges.len() as arc::codec::Scalar).sum::<arc::codec::Scalar>();

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

        let link_count = edges.iter().map(|edges| edges.len()).max().unwrap() as arc::codec::Scalar;

        let fact_count = atom_count * link_count;

        let atom_codec = arc::codec::ScalarCodec::new(unit_count);

        let fact_codec = arc::codec::ScalarCodec::new(link_count);

        return Self { edges, node_count, unit_count, atom_count, fact_count, atom_codec, fact_codec };
    }
}

impl<AtomCodec, FactCodec> arc::model::ModelLike for & Model<AtomCodec, FactCodec>
where 
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = AtomCodec::I, Y = Link, I: arc::model::Tiny>
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

    fn atom_scope(& self, atom: Self::Atom) -> impl Iterator<Item = Self::Fact>
    {
        return self.links(atom).map(move |link| self.encode_fact(atom, link));
    }

    #[allow(unused_variables)]
    fn atom_scope_size(& self, atom: Self::Atom) -> usize
    {
        let node = arc::model::ModelLike::decode_atom_to_node(self, atom);

        let node_index = node as usize;

        return self.edges[node_index].len();
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

impl<AtomCodec, FactCodec> crate::model::ModelLike for & Model<AtomCodec, FactCodec>
where 
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = AtomCodec::I, Y = Link, I: arc::model::Tiny>
{
    fn edges(& self, node: Self::Node) -> & [Self::Node]
    {
        return & self.edges[node as usize];
    }
}

impl<AtomCodec, FactCodec> ModelLike for & Model<AtomCodec, FactCodec>
where 
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = AtomCodec::I, Y = Link, I: arc::model::Tiny>
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

    fn decode_link(& self, node: Self::Node, link: Link) -> Option<Self::Node>
    {
        let node_index = node as usize;

        let edges = & self.edges[node_index];

        let target_node = edges.get(link as usize).copied();

        return target_node;
    }
}