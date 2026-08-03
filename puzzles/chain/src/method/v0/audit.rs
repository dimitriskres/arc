
#[derive(Debug, Clone)]
pub struct Audit<Model>
{
    model: Model
}

impl<Model> Audit<Model>
{
    pub fn new(model: Model) -> Self
    {
        return Self { model };
    }
}

impl<Model, Field> arc::assert::audit::AuditLike<Model, Field> for Audit<Model>
where
    Model: super::model::ModelLike,
    Field: arc::assert::field::FieldLike<Model>
{
    fn get(& mut self, field: & Field, fact: Model::Fact, affirm_atoms: & mut Vec<Model::Atom>, negate_atoms: & mut Vec<Model::Atom>) -> bool 
    {
        let (atom, link) = self.model.decode_fact(fact);

        let (node, unit) = self.model.decode_atom(atom);

        let Some((target_node, direction)) = self.model.decode_link(node, link) else
        {
            return true;
        };

        for target_unit in field.iter(target_node)
        {
            let result = match direction
            {
                super::model::Side::L => target_unit < unit,
                super::model::Side::R => target_unit > unit
            };

            if !result
            {
                continue;
            };

            let target_atom = self.model.encode_atom(target_node, target_unit);

            affirm_atoms.push(target_atom);

            return true;
        };

        negate_atoms.push(atom);

        return false;
    }
}

impl<Model> From<Model> for Audit<Model>
where
    Model: arc::model::ModelLike<Node = arc::codec::Scalar, Unit = arc::codec::Scalar>
{
    fn from(model: Model) -> Self
    {
        return Self::new(model);
    }
}

impl<Model> arc::coerce::revert::Revertible for Audit<Model>
where
    Model: arc::model::ModelLike<Node = arc::codec::Scalar, Unit = arc::codec::Scalar>
{
    fn save(& mut self)
    {
        // No state to save.
    }

    fn load(& mut self) -> bool
    {
        // No state to load.

        return true;
    }
}