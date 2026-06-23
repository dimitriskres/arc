
#[derive(Debug, Clone)]
pub struct State<Model>
where 
    Model: crate::model::ModelLike
{
    pub(crate) negate_nodes: Vec<Model::Node>,
    pub(crate) select_node: Option<Model::Node>,
    pub(crate) select_units: Vec<Model::Unit>
}

impl<Model> State<Model>
where 
    Model: crate::model::ModelLike
{
    pub fn new() -> Self
    {
        let negate_nodes = Vec::with_capacity(64);

        let select_node = None;

        let select_units = Vec::with_capacity(64);

        return Self { negate_nodes, select_node, select_units };
    }

    pub fn negate_nodes(& self) -> & Vec<Model::Node>
    {
        return & self.negate_nodes;
    }

    pub fn select_node(& self) -> Option<Model::Node>
    {
        return self.select_node;
    }

    pub fn select_units(& self) -> & Vec<Model::Unit>
    {
        return & self.select_units;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrackStep<Model>
where 
    Model: crate::model::ModelLike
{
    pub negate_atoms_index: usize,
    pub coerce_units_index: usize,
    pub coerce_node: Model::Node
}

#[derive(Debug, Clone)]
pub struct Track<Model>
where 
    Model: crate::model::ModelLike
{
    pub(crate) negate_atoms: Vec<Model::Atom>,
    pub(crate) coerce_units: Vec<Model::Unit>,
    pub(crate) steps: Vec<TrackStep<Model>>
}

impl<Model> Track<Model>
where 
    Model: crate::model::ModelLike
{
    pub fn new() -> Self
    {
        let negate_atoms = Vec::new();

        let coerce_units = Vec::new();

        let steps = Vec::new();

        return Self { negate_atoms, coerce_units, steps };
    }

    pub fn negate_atoms(& self) -> & Vec<Model::Atom>
    {
        return & self.negate_atoms;
    }

    pub fn coerce_units(& self) -> & Vec<Model::Unit>
    {
        return & self.coerce_units;
    }

    pub fn steps(& self) -> & Vec<TrackStep<Model>>
    {
        return & self.steps;
    }
}

#[derive(Debug, Clone)]
pub struct Solver<Model, Audit, Field, Queue, Cache, Probe> 
where 
    Model: crate::model::ModelLike
{
    pub(crate) inner: crate::assert::solver::Solver<Model, Audit, Field, Queue, Cache>,
    pub(crate) probe: Probe,
    pub(crate) state: State<Model>,
    pub(crate) track: Track<Model>
}

#[derive(Debug, Clone, Copy)]
pub enum AssertResult
{
    Assert { action: crate::assert::action::Action },
    Select,
    Revert
}

#[derive(Debug, Clone, Copy)]
pub enum SelectResult
{
    Coerce,
    Finish
}

#[derive(Debug, Clone, Copy)]
pub enum RevertResult
{
    Select,
    Revert
}

impl<Model, Audit, Field, Queue, Cache, Probe> Solver<Model, Audit, Field, Queue, Cache, Probe>
where 
    Model: crate::model::ModelLike
{
    pub fn new(inner: crate::assert::solver::Solver<Model, Audit, Field, Queue, Cache>, probe: Probe) -> Self
    {
        let state = State::new();
        
        let track = Track::new();

        return Self { inner, probe, state, track };
    }

    pub fn inner(& self) -> & crate::assert::solver::Solver<Model, Audit, Field, Queue, Cache>
    {
        return & self.inner;
    }

    pub fn probe(& self) -> & Probe
    {
        return & self.probe;
    }

    pub fn state(& self) -> & State<Model>
    {
        return & self.state;
    }

    pub fn track(& self) -> & Track<Model>
    {
        return & self.track;
    }
}

impl<Model, Audit, Field, Queue, Cache, Probe> From<Model> for Solver<Model, Audit, Field, Queue, Cache, Probe>
where 
    Model: crate::model::ModelLike + Copy,
    Audit: From<Model>,
    Field: From<Model>,
    Queue: From<Model>,
    Cache: From<Model>,
    Probe: From<Model>,
{
    fn from(model: Model) -> Self
    {
        let inner = model.into();

        let probe = model.into();

        return Self::new(inner, probe);
    }
}

impl<Model, Audit, Field, Queue, Cache, Probe> Solver<Model, Audit, Field, Queue, Cache, Probe>
where 
    Model: crate::model::ModelLike,
    Audit: crate::assert::audit::AuditLike<Model, Field> + crate::coerce::revert::Revertible,
    Field: crate::assert::field::FieldLike<Model>,
    Queue: crate::assert::queue::QueueLike<Model>,
    Cache: crate::assert::cache::CacheLike<Model>,
    Probe: crate::coerce::probe::ProbeLike<Model, Field> + crate::coerce::revert::Revertible
{
    fn handle_assert(& mut self, action: crate::assert::action::Action) -> AssertResult
    {
        if matches!(action, crate::assert::action::Action::Settle | crate::assert::action::Action::Negate)
        {
            self.state.negate_nodes.clear();

            for atom in self.inner.state.negate_atoms.iter().copied()
            {
                self.track.negate_atoms.push(atom);

                let (node, unit) = self.inner.model.decode_atom(atom);

                self.probe.remove(node, unit);

                if !self.state.negate_nodes.contains(& node)
                {
                    self.state.negate_nodes.push(node);
                };
            };
        };

        match crate::solver::SolverLike::handle(& mut self.inner, action)
        {
            Ok(action) =>
            {
                for node in self.state.negate_nodes.iter().copied()
                {
                    let mut units = self.inner.field.iter(node);

                    if units.next().is_none()
                    {
                        return AssertResult::Revert;
                    };
                };

                return AssertResult::Assert { action };
            }
            Err(signal) => 
            {
                match signal
                {
                    crate::assert::signal::Signal::Finish =>
                    {
                        return AssertResult::Select;
                    },
                    crate::assert::signal::Signal::Reject =>
                    {
                        return AssertResult::Revert;
                    }
                };
            }
        };
    }

    fn handle_select(& mut self) -> SelectResult
    {
        self.state.select_units.clear();

        self.probe.select(& self.inner.field, & mut self.state.select_node, & mut self.state.select_units);
        
        if self.state.select_units.is_empty()
        {
            return SelectResult::Finish;
        };

        return SelectResult::Coerce;
    }

    fn handle_coerce(& mut self)
    {
        crate::coerce::revert::Revertible::save(& mut self.inner.audit);

        crate::coerce::revert::Revertible::save(& mut self.probe);

        let negate_atoms_index = self.track.negate_atoms.len();

        let coerce_node = self.state.select_node.unwrap();

        let coerce_units_index = self.track.coerce_units.len();

        let step = TrackStep { negate_atoms_index, coerce_units_index, coerce_node };

        self.track.steps.push(step);

        self.track.coerce_units.extend_from_slice(& self.state.select_units);

        self.probe.remove_many(coerce_node, & self.state.select_units);

        for select_unit in self.state.select_units.iter().copied()
        {
            let atom = self.inner.model.encode_atom(coerce_node, select_unit);

            self.track.negate_atoms.push(atom);
            
            self.inner.remove(atom);
        };
    }
    
    fn handle_revert(& mut self) -> RevertResult
    {
        let Some(TrackStep { negate_atoms_index, coerce_units_index, coerce_node }) = self.track.steps.pop() else
        {
            return RevertResult::Revert;
        };

        if !crate::coerce::revert::Revertible::load(& mut self.probe)
        {
            return RevertResult::Revert;
        };

        if !crate::coerce::revert::Revertible::load(& mut self.inner.audit)
        {
            return RevertResult::Revert;
        };

        let negate_atoms = & self.track.negate_atoms[negate_atoms_index..];

        for atom in negate_atoms.iter().copied()
        {
            self.inner.insert(atom);
        };

        self.track.negate_atoms.truncate(negate_atoms_index);

        self.state.select_units.clear();

        let coerce_units = & self.track.coerce_units[coerce_units_index..];

        for unit in self.inner.field().iter(coerce_node)
        {
            if coerce_units.contains(& unit)
            {
                continue;
            };

            self.state.select_units.push(unit);
        };

        self.probe.remove_many(coerce_node, & self.state.select_units);

        for unit in self.state.select_units.iter().copied()
        {
            let atom = self.inner.model().encode_atom(coerce_node, unit);

            self.inner.remove(atom);

            self.track.negate_atoms.push(atom);
        };

        self.track.coerce_units.truncate(coerce_units_index);

        return RevertResult::Select;
    }
}

impl<Model, Audit, Field, Queue, Cache, Probe> crate::solver::SolverLike for Solver<Model, Audit, Field, Queue, Cache, Probe>
where 
    Model: crate::model::ModelLike,
    Audit: crate::assert::audit::AuditLike<Model, Field> + crate::coerce::revert::Revertible,
    Field: crate::assert::field::FieldLike<Model>,
    Queue: crate::assert::queue::QueueLike<Model>,
    Cache: crate::assert::cache::CacheLike<Model>,
    Probe: crate::coerce::probe::ProbeLike<Model, Field> + crate::coerce::revert::Revertible
{
    type Action = crate::coerce::action::Action;
    type Signal = crate::coerce::signal::Signal;

    fn handle(& mut self, action: Self::Action) -> Result<Self::Action, Self::Signal> 
    {
        match action
        {
            crate::coerce::action::Action::Assert { action } => 
            {
                match self.handle_assert(action)
                {
                    AssertResult::Assert { action } =>
                    {
                        return Ok(crate::coerce::action::Action::Assert { action });
                    }
                    AssertResult::Select =>
                    {
                        return Ok(crate::coerce::action::Action::Select);
                    }
                    AssertResult::Revert =>
                    {
                        return Ok(crate::coerce::action::Action::Revert);
                    }
                };
            },
            crate::coerce::action::Action::Select => 
            {
                match self.handle_select()
                {
                    SelectResult::Coerce =>
                    {
                        return Ok(crate::coerce::action::Action::Coerce);
                    }
                    SelectResult::Finish =>
                    {
                        return Err(crate::coerce::signal::Signal::Finish);
                    }
                };
            },
            crate::coerce::action::Action::Coerce =>
            {
                self.handle_coerce();

                return Ok(crate::coerce::action::Action::default());
            },
            crate::coerce::action::Action::Revert => 
            {
                match self.handle_revert()
                {
                    RevertResult::Select =>
                    {
                        return Ok(crate::coerce::action::Action::default());
                    }
                    RevertResult::Revert =>
                    {
                        return Err(crate::coerce::signal::Signal::Revert);
                    }
                };
            }
        };
    }
}
