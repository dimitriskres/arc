
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

    pub fn model(& self) -> & Model
    {
        return & self.model;
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

        let target_node = self.model.decode_link(node, link);

        for target_unit in field.iter(target_node)
        {
            if unit == target_unit
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
{
    fn from(model: Model) -> Self
    {
        return Self::new(model);
    }
}

impl<Model> arc::coerce::revert::Revertible for Audit<Model>

{
    #[inline(always)]
    fn save(& mut self)
    {
        // no-op
    }

    #[inline(always)]
    fn load(& mut self) -> bool
    {
        // no-op

        return true;
    }
}