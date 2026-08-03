
pub trait ModelLike: arc::model::ModelLike
{
	fn edges(& self, node: Self::Node) -> & [Self::Node]; 
}