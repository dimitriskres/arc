
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
        let (origin_node, edge) = self.model.decode_fact(fact);

        let Some(target_node) = self.model.decode_link(origin_node, edge) else
        {
            return true;
        };

        let mut target_units = field.iter(target_node);

        let Some(first_unit) = target_units.next() else
        {
            return false;
        };

        let first_atom = self.model.encode_atom(target_node, first_unit);

        affirm_atoms.push(first_atom);

        if let Some(second_unit) = target_units.next()
        {
            let second_atom = self.model.encode_atom(target_node, second_unit);

            affirm_atoms.push(second_atom);

            return true;
        };

        let origin_atom = self.model.encode_atom(origin_node, first_unit);

        if field.active(origin_atom)
        {
            negate_atoms.push(origin_atom);
        };

        return true;
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