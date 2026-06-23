
pub trait Tiny
where 
    Self: Sized + std::fmt::Debug + Clone + Copy + std::hash::Hash + PartialEq + Eq + PartialOrd + Ord + Send + Sync
{

}

impl Tiny for crate::codec::Scalar
{

}

pub trait ModelLike
{
    type Node: Tiny;
    type Unit: Tiny;
    type Atom: Tiny;
    type Fact: Tiny;

    fn node_count(& self) -> usize;

    fn nodes(& self) -> impl Iterator<Item = Self::Node>;

    fn unit_count(& self, node: Self::Node) -> usize;

    fn units(& self, node: Self::Node) -> impl Iterator<Item = Self::Unit>;

    fn atom_count(& self) -> usize;

    fn atom_scope(& self, atom: Self::Atom) -> impl Iterator<Item = Self::Fact>;

    fn atom_scope_size(& self, atom: Self::Atom) -> usize;

    fn atoms(& self) -> impl Iterator<Item = Self::Atom>;

    fn encode_atom(& self, node: Self::Node, unit: Self::Unit) -> Self::Atom;

    fn decode_atom_to_node(& self, atom: Self::Atom) -> Self::Node;

    fn decode_atom_to_unit(& self, atom: Self::Atom) -> Self::Unit;

    fn decode_atom(& self, atom: Self::Atom) -> (Self::Node, Self::Unit);

    fn fact_count(& self) -> usize;

    fn fact_scope(& self, fact: Self::Fact) -> impl Iterator<Item = Self::Atom>;

    fn fact_scope_size(& self, fact: Self::Fact) -> usize;

    fn facts(& self) -> impl Iterator<Item = Self::Fact>;
}
