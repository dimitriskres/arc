
pub trait AuditLike<Model, Field>
where 
    Model: crate::model::ModelLike
{
    fn get(& mut self, field: & Field, fact: Model::Fact, affirm_atoms: & mut Vec<Model::Atom>, negate_atoms: & mut Vec<Model::Atom>) -> bool;
}

