
#[derive(Debug, Clone)]
pub struct State<Model>
where 
    Model: crate::model::ModelLike
{
    pub(crate) locate_fact: Option<Model::Fact>,
    pub(crate) affirm_atoms: Vec<Model::Atom>,
    pub(crate) negate_atoms: Vec<Model::Atom>
}

impl<Model> State<Model>
where 
    Model: crate::model::ModelLike
{
    pub fn new() -> Self
    {
        let locate_fact = None;

        let affirm_atoms = Vec::with_capacity(64);
        let negate_atoms = Vec::with_capacity(64);

        return Self { locate_fact, affirm_atoms, negate_atoms };
    }

    pub fn locate_fact(& self) -> Option<Model::Fact>
    {
        return self.locate_fact;
    }

    pub fn affirm_atoms(& self) -> & Vec<Model::Atom>
    {
        return & self.affirm_atoms;
    }

    pub fn negate_atoms(& self) -> & Vec<Model::Atom>
    {
        return & self.negate_atoms;
    }
}

#[derive(Debug, Clone)]
pub struct Solver<Model, Audit, Field, Queue, Cache> 
where 
    Model: crate::model::ModelLike
{
    pub(crate) model: Model,
    pub(crate) audit: Audit,
    pub(crate) field: Field,
    pub(crate) queue: Queue,
    pub(crate) cache: Cache,
    pub(crate) state: State<Model>
}

#[derive(Debug, Clone, Copy)]
pub enum NextResult
{
    Locate,
    Finish
}

#[derive(Debug, Clone, Copy)]
pub enum LocateResult
{
    Settle,
    Negate,
    Reject
}

impl<Model, Audit, Field, Queue, Cache> Solver<Model, Audit, Field, Queue, Cache>
where 
    Model: crate::model::ModelLike
{
    pub fn new(model: Model, audit: Audit, field: Field, queue: Queue, cache: Cache) -> Self
    {
        let state = State::new();

        return Self { model, audit, field, queue, cache, state };
    }

    pub fn model(& self) -> & Model
    {
        return & self.model;
    }

    pub fn audit(& self) -> & Audit
    {
        return & self.audit;
    }

    pub fn field(& self) -> & Field
    {
        return & self.field;
    }

    pub fn queue(& self) -> & Queue
    {
        return & self.queue;
    }

    pub fn cache(& self) -> & Cache
    {
        return & self.cache;
    }

    pub fn state(& self) -> & State<Model>
    {
        return & self.state;
    }

    pub fn remove(& mut self, atom: Model::Atom)
    where
        Field: crate::assert::field::FieldLike<Model>,
        Queue: crate::assert::queue::QueueLike<Model>,
        Cache: crate::assert::cache::CacheLike<Model>
    {
        self.field.remove(atom);
    
        let emit = |fact| self.queue.put(fact);

        self.cache.remove(atom, emit);
    }

    pub fn insert(& mut self, atom: Model::Atom)
    where
        Field: crate::assert::field::FieldLike<Model>,
        Queue: crate::assert::queue::QueueLike<Model>
    {
        self.field.insert(atom);

        for fact in self.model.atom_scope(atom)
        {
            self.queue.put(fact);
        };
    }
}

impl<Model, Audit, Field, Queue, Cache> From<Model> for Solver<Model, Audit, Field, Queue, Cache>
where 
    Model: crate::model::ModelLike + Copy,
    Audit: From<Model>,
    Field: From<Model>,
    Queue: From<Model>,
    Cache: From<Model>
{
    fn from(model: Model) -> Self
    {
        let audit = model.into();
        let field = model.into();
        let queue = model.into();
        let cache = model.into();

        return Self::new(model, audit, field, queue, cache);
    }
}

impl<Model, Audit, Field, Queue, Cache> crate::assert::solver::Solver<Model, Audit, Field, Queue, Cache>
where 
    Model: crate::model::ModelLike,
    Audit: crate::assert::audit::AuditLike<Model, Field>,
    Field: crate::assert::field::FieldLike<Model>,
    Queue: crate::assert::queue::QueueLike<Model>,
    Cache: crate::assert::cache::CacheLike<Model>
{
    #[allow(unused_variables)]
    fn handle_next(& mut self) -> crate::assert::solver::NextResult
    {
        while let Some(fact) = self.queue.get()
        {
            if self.cache.active(fact)
            {
                continue;
            };

            if !self.model.fact_scope(fact).all(|atom| self.field.active(atom))
            {
                continue;
            };

            self.state.locate_fact = Some(fact);

            return NextResult::Locate;
        };

        return NextResult::Finish;
    }

    fn handle_locate(& mut self) -> crate::assert::solver::LocateResult
    {
        let fact = self.state.locate_fact.unwrap();

        self.state.affirm_atoms.clear();

        self.state.negate_atoms.clear();

        if self.audit.get(& self.field, fact, & mut self.state.affirm_atoms, & mut self.state.negate_atoms)
        {
            return LocateResult::Settle;
        };

        if self.model.fact_scope_size(fact) == 1
        {
            return LocateResult::Negate;
        };

        return LocateResult::Reject;
    }

    fn handle_affirm(& mut self)
    {
        let fact = self.state.locate_fact.unwrap();

        self.cache.insert(fact, & self.state.affirm_atoms);
    }

    fn handle_negate(& mut self)
    {
        for index in 0..self.state.negate_atoms.len()
        {
            let atom = self.state.negate_atoms[index];

            self.remove(atom);
        };
    }

    fn handle_settle(& mut self)
    {
        self.handle_affirm();

        self.handle_negate();
    }
}

impl<Model, Audit, Field, Queue, Cache> crate::solver::SolverLike for crate::assert::solver::Solver<Model, Audit, Field, Queue, Cache>
where 
    Model: crate::model::ModelLike,
    Audit: crate::assert::audit::AuditLike<Model, Field>,
    Field: crate::assert::field::FieldLike<Model>,
    Queue: crate::assert::queue::QueueLike<Model>,
    Cache: crate::assert::cache::CacheLike<Model>
{
    type Action = crate::assert::action::Action;
    type Signal = crate::assert::signal::Signal;

    fn handle(& mut self, action: Self::Action) -> Result<Self::Action, Self::Signal> 
    {
        match action
        {
            crate::assert::action::Action::Next => 
            {
                match self.handle_next()
                {
                    crate::assert::solver::NextResult::Locate =>
                    {
                        return Ok(crate::assert::action::Action::Locate);
                    }
                    crate::assert::solver::NextResult::Finish => 
                    {
                        return Err(crate::assert::signal::Signal::Finish);
                    }
                };
            },
            crate::assert::action::Action::Locate => 
            {
                match self.handle_locate()
                {
                    crate::assert::solver::LocateResult::Settle =>
                    {
                        return Ok(crate::assert::action::Action::Settle);
                    }
                    crate::assert::solver::LocateResult::Negate =>
                    {
                        return Ok(crate::assert::action::Action::Negate);
                    }
                    crate::assert::solver::LocateResult::Reject => 
                    {
                        return Err(crate::assert::signal::Signal::Reject);
                    }
                };
            },
            crate::assert::action::Action::Settle =>
            {
                self.handle_settle();

                return Ok(crate::assert::action::Action::Next);
            },
            crate::assert::action::Action::Negate =>
            {
                self.handle_negate();

                return Ok(crate::assert::action::Action::Next);
            }
        };
    }
}
