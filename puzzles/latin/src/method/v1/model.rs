
pub type Link = arc::codec::Scalar;
pub type Unit = arc::codec::Scalar;

pub trait ModelLike: crate::model::ModelLike
{
    #[allow(unused_variables)]
    fn links(& self, node: Self::Node) -> impl Iterator<Item = Link>;

    fn encode_fact(& self, node: Self::Node, link: Link) -> Self::Fact;

    fn decode_fact_to_node(& self, fact: Self::Fact) -> Self::Node;

    fn decode_fact_to_link(& self, fact: Self::Fact) -> Link;

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Node, Link);

    fn decode_link(& self, node: Self::Node, link: Link) -> Self::Node
    {
        let size = self.unit_count(node) as arc::codec::Scalar;

        let (row, col) = self.decode_node(node);

        let target_col;
        let target_row;

        if let Some(link) = link.checked_sub(size - 1)
        {
            target_row = link + (link >= row) as crate::model::Row;
            target_col = col;
        }
        else
        {
            target_row = row;
            target_col = link + (link >= col) as crate::model::Col;
        };

        let target_node = self.encode_node(target_row, target_col);

        return target_node;
    }
}

#[derive(Clone)]
pub struct Model<NodeCodec, AtomCodec, FactCodec>
{
    pub size: arc::codec::Scalar,
    pub node_count: arc::codec::Scalar,
    pub unit_count: arc::codec::Scalar,
    pub atom_count: arc::codec::Scalar,
    pub link_count: arc::codec::Scalar,
    pub fact_count: arc::codec::Scalar,
    pub node_codec: NodeCodec,
    pub atom_codec: AtomCodec,
    pub fact_codec: FactCodec,
    pub rate_codec: arc::codec::ScalarCodec
}

pub type ObjectNode = arc::codec::Scalar;
pub type ObjectAtom = arc::codec::Object<ObjectNode, Unit>;
pub type ObjectFact = arc::codec::Object<ObjectNode, Link>;

pub type ObjectNodeCodec = arc::codec::ScalarCodec;
pub type ObjectAtomCodec = arc::codec::ObjectCodec<ScalarNode, Unit>;
pub type ObjectFactCodec = arc::codec::ObjectCodec<ObjectNode, Link>;

pub type ObjectModel = Model<ObjectNodeCodec, ObjectAtomCodec, ObjectFactCodec>;

impl ObjectModel
{
    pub fn new(size: arc::codec::Scalar) -> Self
    {
        let node_count = size * size;

        let unit_count = size;

        let atom_count = node_count * unit_count;

        let link_count = (size - 1) * 2;

        let fact_count = node_count * link_count;

        let rate_codec = arc::codec::ScalarCodec::new(size - 1);

        let node_codec = arc::codec::ScalarCodec::new(size);

        let atom_codec = arc::codec::ObjectCodec::new();

        let fact_codec = arc::codec::ObjectCodec::new();

        return Self { size, node_count, unit_count, atom_count, link_count, fact_count, node_codec, atom_codec, fact_codec, rate_codec };
    }
}

pub type ScalarNode = arc::codec::Scalar;
pub type ScalarAtom = arc::codec::Scalar;
pub type ScalarFact = arc::codec::Scalar;

pub type ScalarNodeCodec = arc::codec::ScalarCodec;
pub type ScalarAtomCodec = arc::codec::ScalarCodec;
pub type ScalarFactCodec = arc::codec::ScalarCodec;

pub type ScalarModel = Model<ScalarNodeCodec, ScalarAtomCodec, ScalarFactCodec>;

impl ScalarModel
{
    pub fn new(size: arc::codec::Scalar) -> Self
    {
        let node_count = size * size;

        let unit_count = size;

        let atom_count = node_count * unit_count;

        let link_count = (size - 1) * 2;

        let fact_count = node_count * link_count;

        let rate_codec = arc::codec::ScalarCodec::new(size - 1);

        let node_codec = arc::codec::ScalarCodec::new(size);

        let atom_codec = arc::codec::ScalarCodec::new(unit_count);

        let fact_codec = arc::codec::ScalarCodec::new(link_count);

        return Self { size, node_count, unit_count, atom_count, link_count, fact_count, node_codec, atom_codec, fact_codec, rate_codec };
    }
}

impl<NodeCodec, AtomCodec, FactCodec> arc::model::ModelLike for & Model<NodeCodec, AtomCodec, FactCodec>
where 
    NodeCodec: arc::codec::CodecLike<X = crate::model::Row, Y = crate::model::Col, I: arc::model::Tiny>,
    AtomCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Link, I: arc::model::Tiny>
{
    type Node = NodeCodec::I;
    type Unit = NodeCodec::Y;
    type Atom = AtomCodec::I;
    type Fact = FactCodec::I;

    fn node_count(& self) -> usize
    {
        return self.node_count as usize;
    }

    fn nodes(& self) -> impl Iterator<Item = Self::Node>
    {
        return (0..self.size).flat_map(move |row| (0..self.size).map(move |col| arc::codec::CodecLike::encode(& self.node_codec, row, col)));
    }

    #[allow(unused_variables)]
    fn unit_count(& self, node: Self::Node) -> usize
    {
        return self.unit_count as usize;
    }

    #[allow(unused_variables)]
    fn units(& self, node: Self::Node) -> impl Iterator<Item = Self::Unit>
    {
        return 0..self.size;
    }

    fn atom_count(& self) -> usize
    {
        return self.atom_count as usize;
    }

    fn atom_scope(& self, atom: Self::Atom) -> impl Iterator<Item = Self::Fact>
    {
        let node = self.decode_atom_to_node(atom);

        return self.links(node).map(move |link| self.encode_fact(node, link));
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
        return self.nodes().flat_map(move |node| self.links(node).map(move |link| arc::codec::CodecLike::encode(& self.fact_codec, node, link)));
    }
}

impl<NodeCodec, AtomCodec, FactCodec> crate::model::ModelLike for & Model<NodeCodec, AtomCodec, FactCodec>
where 
    NodeCodec: arc::codec::CodecLike<X = crate::model::Row, Y = crate::model::Col, I: arc::model::Tiny>,
    AtomCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Link, I: arc::model::Tiny>
{
    fn size(& self) -> arc::codec::Scalar
    {
        return self.size;
    }

    fn decode_node_to_row(& self, node: Self::Node) -> crate::model::Row
    {
        return arc::codec::CodecLike::decode_to_x(& self.node_codec, node);
    }

    fn decode_node_to_col(& self, node: Self::Node) -> crate::model::Col
    {
        return arc::codec::CodecLike::decode_to_y(& self.node_codec, node);
    }

    fn decode_node(& self, node: Self::Node) -> (crate::model::Row, crate::model::Col)
    {
        return arc::codec::CodecLike::decode(& self.node_codec, node);
    }

    fn encode_node(& self, row: crate::model::Row, col: crate::model::Col) -> Self::Node
    {
        return arc::codec::CodecLike::encode(& self.node_codec, row, col);
    }
}

impl<NodeCodec, AtomCodec, FactCodec> ModelLike for & Model<NodeCodec, AtomCodec, FactCodec>
where 
    NodeCodec: arc::codec::CodecLike<X = crate::model::Row, Y = crate::model::Col, I: arc::model::Tiny>,
    AtomCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Unit, I: arc::model::Tiny>,
    FactCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Link, I: arc::model::Tiny>
{
    #[allow(unused_variables)]
    fn links(& self, node: Self::Node) -> impl Iterator<Item = Link>
    {
        return 0..self.link_count;
    }

    fn encode_fact(& self, node: Self::Node, link: Link) -> Self::Fact
    {
        return arc::codec::CodecLike::encode(& self.fact_codec, node, link);
    }

    fn decode_fact_to_node(& self, fact: Self::Fact) -> Self::Node
    {
        return arc::codec::CodecLike::decode_to_x(& self.fact_codec, fact);
    }

    fn decode_fact_to_link(& self, fact: Self::Fact) -> Link
    {
        return arc::codec::CodecLike::decode_to_y(& self.fact_codec, fact);
    }

    fn decode_fact(& self, fact: Self::Fact) -> (Self::Node, Link)
    {
        return arc::codec::CodecLike::decode(& self.fact_codec, fact);
    }
}

impl<NodeCodec, AtomCodec, FactCodec> arc::assert::cache::CacheBiModelLike for & Model<NodeCodec, AtomCodec, FactCodec>
where
    NodeCodec: arc::codec::CodecLike<X = crate::model::Row, Y = crate::model::Col, I = ScalarNode>,
    AtomCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Unit, I = ScalarAtom>,
    FactCodec: arc::codec::CodecLike<X = NodeCodec::I, Y = Link, I = ScalarFact>
{
    #[allow(unused_variables)]
    fn edge_count(& self, atom: Self::Atom) -> usize
    {
        return self.link_count as usize;
    }

    fn edge_start(& self, atom: Self::Atom) -> usize
    {
        return atom as usize * self.edge_count(atom);
    }

    fn encode_edge(& self, atom: Self::Atom, fact: Self::Fact) -> usize
    {
        let target_node = arc::model::ModelLike::decode_atom_to_node(self, atom);

        let (origin_node, origin_link) = ModelLike::decode_fact(self, fact);

        debug_assert_eq!(ModelLike::decode_link(self, origin_node, origin_link), target_node);

        let (origin_row, origin_col) = crate::model::ModelLike::decode_node(self, origin_node);

        let (target_row, target_col) = crate::model::ModelLike::decode_node(self, target_node);

        let link_count = self.size - 1;

        let target_link = if origin_row == target_row
        {
            origin_col - (origin_col >= target_col) as crate::model::Col
        }
        else
        {
            debug_assert_eq!(origin_col, target_col);

            origin_row - (origin_row >= target_row) as crate::model::Row + link_count
        };

        return target_link as usize;
    }

    fn decode_edge(& self, atom: Self::Atom, edge: usize) -> Self::Fact
    {
        let target_node = arc::model::ModelLike::decode_atom_to_node(self, atom);

        let (target_row, target_col) = crate::model::ModelLike::decode_node(self, target_node);

        let target_link = edge as arc::codec::Scalar;

        let link_count = self.size - 1;

        debug_assert!(target_link < self.link_count);

        let origin_node;

        let origin_link;

        if target_link < link_count
        {
            let origin_col = target_link + (target_link >= target_col) as crate::model::Col;

            origin_node = crate::model::ModelLike::encode_node(self, target_row, origin_col);

            origin_link = target_col - (target_col >= origin_col) as Link;
        }
        else
        {
            let row_link = target_link - link_count;

            let origin_row = row_link + (row_link >= target_row) as crate::model::Row;

            origin_node = crate::model::ModelLike::encode_node(self, origin_row, target_col);

            origin_link = target_row - (target_row >= origin_row) as Link + link_count;
        };

        return ModelLike::encode_fact(self, origin_node, origin_link);
    }
}