
pub trait SolverLike
{
    type Action;
    type Signal;

    fn handle(& mut self, action: Self::Action) -> Result<Self::Action, Self::Signal>;

    fn solution(& mut self) -> Solution<'_, Self>
    where 
        Self: Sized,
        Self::Action: Default
    {
        let action = Default::default();

        return Solution::new(self, action);
    }
}

pub struct Solution<'solver, Solver>
where 
    Solver: SolverLike
{
    solver: &'solver mut Solver,
    action: std::mem::MaybeUninit<Solver::Action>
}

impl<'solver, Solver> Solution<'solver, Solver>
where 
    Solver: SolverLike
{
    fn new(solver: &'solver mut Solver, action: Solver::Action) -> Self
    {
        return Self { solver, action: std::mem::MaybeUninit::new(action) };
    }

    pub fn solver(& self) -> & Solver
    {
        return & self.solver;
    }

    pub unsafe fn step(& mut self) -> Result<& Solver::Action, Solver::Signal>
    {
        let action = unsafe { self.action.assume_init_read() };

        self.action.write(self.solver.handle(action)?);

        let action = unsafe { self.action.assume_init_ref() };

        return Ok(action);
    }

    pub fn finish(& mut self) -> Solver::Signal
    {
        loop
        {
            if let Err(signal) = unsafe { self.step() }
            {
                return signal;
            };
        };
    }
}

impl<'solver, Solver> IntoIterator for Solution<'solver, Solver>
where 
    Solver: SolverLike,
    Solver::Action: Clone
{
    type Item = Result<Solver::Action, Solver::Signal>;
    type IntoIter = SolutionIterator<'solver, Solver>;

    fn into_iter(self) -> Self::IntoIter 
    {
        return SolutionIterator::new(self);    
    }
}

pub struct SolutionIterator<'solver, Solver>
where 
    Solver: SolverLike
{
    solution: Solution<'solver, Solver>,
    complete: bool
}

impl<'solver, Solver> SolutionIterator<'solver, Solver>
where 
    Solver: SolverLike
{
    fn new(solution: Solution<'solver, Solver>) -> Self
    {
        return Self { solution, complete: false }
    }

    pub fn solution(& self) -> & Solution<'solver, Solver>
    {
        return & self.solution;
    }
}

impl<'solver, Solver> Iterator for SolutionIterator<'solver, Solver>
where 
    Solver: SolverLike,
    Solver::Action: Clone
{
    type Item = Result<Solver::Action, Solver::Signal>;

    fn next(& mut self) -> Option<Self::Item> 
    {
        if self.complete
        {
            return None;
        };

        let result = unsafe { self.solution.step() }.cloned();    

        self.complete = result.is_err();

        return Some(result);
    }
}
