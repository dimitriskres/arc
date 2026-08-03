
pub type Row = arc::codec::Scalar;
pub type Col = arc::codec::Scalar;

pub trait ModelLike: arc::model::ModelLike
{
	fn size(& self) -> arc::codec::Scalar;

	fn decode_node_to_row(& self, node: Self::Node) -> Row;

    fn decode_node_to_col(& self, node: Self::Node) -> Col;

    fn decode_node(& self, node: Self::Node) -> (Row, Col);

    fn encode_node(& self, row: Row, col: Col) -> Self::Node;
}