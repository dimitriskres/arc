
pub const LINK_COUNT: arc::codec::Scalar = 2;

type Node = arc::codec::Scalar;
type Unit = arc::codec::Scalar;
type Link = arc::codec::Scalar;

pub trait ModelLike: arc::model::ModelLike<Node = Node, Unit = Unit>
{
    #[allow(unused_variables)]
    fn links(& self, atom: Self::Atom) -> impl Iterator<Item = Link>
    {
        return 0..LINK_COUNT;
    }

    fn encode_fact(& self, atom: Self::Atom, link: Link) -> Self::Fact;

    fn decode_fact_to_atom(& self, fact: Self::Fact) -> Self::Atom;

    fn decode_fact_to_link(& self, fact: Self::Fact) -> Link;

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Atom, Link);
}

#[derive(Clone, Copy)]
pub struct Model<AtomCodec, FactCodec>
{
    pub node_count: arc::codec::Scalar,
    pub unit_count: arc::codec::Scalar,
    pub atom_codec: AtomCodec,
    pub fact_codec: FactCodec
}

impl<AtomCodec, FactCodec> Model<AtomCodec, FactCodec>
{
    pub fn build(node_count: arc::codec::Scalar, unit_count: arc::codec::Scalar, atom_codec: AtomCodec, fact_codec: FactCodec) -> Self
    {
        return Self { node_count, unit_count, atom_codec, fact_codec };
    }
}   

pub type ObjectAtom = arc::codec::Object<Node, Unit>;
pub type ObjectFact = arc::codec::Object<ObjectAtom, Link>;

pub type ObjectAtomCodec = arc::codec::ObjectCodec<Node, Unit>;
pub type ObjectFactCodec = arc::codec::ObjectCodec<ObjectAtom, Link>;

pub type ObjectModel = Model<ObjectAtomCodec, ObjectFactCodec>;

impl ObjectModel
{
    pub fn new(node_count: arc::codec::Scalar, unit_count: arc::codec::Scalar) -> Self
    {
        let atom_codec = arc::codec::ObjectCodec::new();

        let fact_codec = arc::codec::ObjectCodec::new();

        return Self { node_count, unit_count, atom_codec, fact_codec };
    }
}

pub type ScalarAtom = arc::codec::Scalar;
pub type ScalarFact = arc::codec::Scalar;

pub type ScalarAtomCodec = arc::codec::UniformCodec;
pub type ScalarFactCodec = arc::codec::UniformCodec;

pub type ScalarModel = Model<ScalarAtomCodec, ScalarFactCodec>;

impl ScalarModel
{
    pub fn new(node_count: arc::codec::Scalar, unit_count: arc::codec::Scalar) -> Self
    {
        let atom_codec = arc::codec::UniformCodec::new(unit_count);

        let fact_codec = arc::codec::UniformCodec::new(LINK_COUNT);

        return Self { node_count, unit_count, atom_codec, fact_codec };
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
        return self.node_count as usize * self.unit_count as usize;
    }

    fn atom_scope(& self, atom: Self::Atom) -> impl Iterator<Item = Self::Fact>
    {
        return self.links(atom).map(move |link| self.encode_fact(atom, link));
    }

    #[allow(unused_variables)]
    fn atom_scope_size(& self, atom: Self::Atom) -> usize
    {
        return LINK_COUNT as usize;
    }

    fn atoms(& self) -> impl Iterator<Item = Self::Atom>
    {
        return (0..self.node_count).flat_map(move |node| (0..self.unit_count).map(move |unit| arc::codec::CodecLike::encode(& self.atom_codec, node, unit)));
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

    #[allow(unused_variables)]
    fn fact_count(& self) -> usize
    {
        return self.atom_count() * LINK_COUNT as usize;
    }

    fn fact_scope(& self, fact: Self::Fact) -> impl Iterator<Item = Self::Atom>
    {
        let atom = self.decode_fact_to_atom(fact);

        return std::iter::once(atom);
    }

    #[allow(unused_variables)]
    fn fact_scope_size(& self, fact: Self::Fact) -> usize
    {
        return 1;
    }

    #[allow(unused_variables)]
    fn facts(& self) -> impl Iterator<Item = Self::Fact>
    {
        return self.atoms().flat_map(move |atom| self.links(atom).map(move |link| self.encode_fact(atom, link)));
    }
}

impl<AtomCodec, FactCodec> ModelLike for & Model<AtomCodec, FactCodec>
where 
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = AtomCodec::I, Y = Link, I: arc::model::Tiny>
{
    fn encode_fact(& self, atom: Self::Atom, link: Link) -> Self::Fact
    {
        let atom = arc::codec::CodecLike::encode(& self.atom_codec, arc::codec::CodecLike::decode_to_x(& self.atom_codec, atom), arc::codec::CodecLike::decode_to_y(& self.atom_codec, atom));

        return arc::codec::CodecLike::encode(& self.fact_codec, atom, link);
    }

    fn decode_fact_to_atom(& self, fact: Self::Fact) -> Self::Atom
    {
        let atom = arc::codec::CodecLike::decode_to_x(& self.fact_codec, fact);

        return arc::codec::CodecLike::encode(& self.atom_codec, arc::codec::CodecLike::decode_to_x(& self.atom_codec, atom), arc::codec::CodecLike::decode_to_y(& self.atom_codec, atom));
    }

    fn decode_fact_to_link(& self, fact: Self::Fact) -> Link
    {
        return arc::codec::CodecLike::decode_to_y(& self.fact_codec, fact);
    }

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Atom, Link)
    {
        let atom = arc::codec::CodecLike::decode_to_x(& self.fact_codec, fact);

        let link = arc::codec::CodecLike::decode_to_y(& self.fact_codec, fact);

        return (arc::codec::CodecLike::encode(& self.atom_codec, arc::codec::CodecLike::decode_to_x(& self.atom_codec, atom), arc::codec::CodecLike::decode_to_y(& self.atom_codec, atom)), link);
    }
}

impl<AtomCodec, FactCodec> arc::assert::cache::CacheBiModelLike for & Model<AtomCodec, FactCodec>
where 
    AtomCodec: arc::codec::CodecLike<X = Node, Y = Unit, I = ScalarAtom>,
    FactCodec: arc::codec::CodecLike<X = AtomCodec::I, Y = Link, I = ScalarFact>
{
    #[allow(unused_variables)]
    fn edge_count(& self, atom: Self::Atom) -> usize 
    {
        return LINK_COUNT as usize * self.unit_count as usize * 2;
    }

    fn edge_start(& self, atom: Self::Atom) -> usize
    {
        return atom as usize * self.edge_count(atom);
    }

    #[allow(unused_variables)]
    fn encode_edge(& self, atom: Self::Atom, fact: Self::Fact) -> usize
    {
        let origin_atom = ModelLike::decode_fact_to_atom(self, fact);

        let node = arc::model::ModelLike::decode_atom_to_node(self, origin_atom);

        let link = ModelLike::decode_fact_to_link(self, fact);

        let rate = LINK_COUNT * self.unit_count;

        let start = rate * node;

        let shift = fact - start;

        let edge = rate * link + shift;

        return edge as usize;
    }

    fn decode_edge(& self, atom: Self::Atom, edge: usize) -> Self::Fact
    {
        let edge = edge as arc::codec::Scalar;

        let node = arc::model::ModelLike::decode_atom_to_node(self, atom);

        let rate = LINK_COUNT * self.unit_count;

        let link = edge / rate;

        let shift = edge - rate * link;

        let node = node - LINK_COUNT * link + 1;

        let fact = rate * node + shift;

        return fact;
    }
}
