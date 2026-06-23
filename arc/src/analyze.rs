
#[derive(Debug, Clone, Copy)]
pub struct SolveReport
{
    pub signal: crate::coerce::signal::Signal,
    pub duration: std::time::Duration
}

pub fn solve<Model, Audit, Field, Queue, Cache, Probe>(model: Model) -> SolveReport
where
    Model: crate::model::ModelLike + Copy,
    Audit: crate::assert::audit::AuditLike<Model, Field> + crate::coerce::revert::Revertible,
    Field: crate::assert::field::FieldLike<Model>,
    Queue: crate::assert::queue::QueueLike<Model>,
    Cache: crate::assert::cache::CacheLike<Model>,
    Probe: crate::coerce::probe::ProbeLike<Model, Field> + crate::coerce::revert::Revertible,
    crate::coerce::solver::Solver<Model, Audit, Field, Queue, Cache, Probe>: From<Model>
{
    let mut solver = crate::coerce::solver::Solver::from(model);

    let mut solution = crate::solver::SolverLike::solution(& mut solver);

    let instant = std::time::Instant::now();

    let signal = solution.finish();

    let duration = instant.elapsed();

    let report = SolveReport { signal, duration };

    return report;
}

#[derive(Debug, Clone, Copy)]
pub struct GaugeReport
{
    pub nexts: usize,
    pub locates: usize,
    pub settles: usize,
    pub negates: usize,
    pub selects: usize,
    pub coerces: usize,
    pub reverts: usize,
    pub duration: std::time::Duration
}

pub fn gauge<Model, Audit, Field, Queue, Cache, Probe>(model: Model, timeout: std::time::Duration) -> Option<GaugeReport>
where
    Model: crate::model::ModelLike + Copy,
    Audit: crate::assert::audit::AuditLike<Model, Field> + crate::coerce::revert::Revertible,
    Field: crate::assert::field::FieldLike<Model>,
    Queue: crate::assert::queue::QueueLike<Model>,
    Cache: crate::assert::cache::CacheLike<Model>,
    Probe: crate::coerce::probe::ProbeLike<Model, Field> + crate::coerce::revert::Revertible,
    crate::coerce::solver::Solver<Model, Audit, Field, Queue, Cache, Probe>: From<Model>
{
    let mut solver = crate::coerce::solver::Solver::from(model);

    let mut solution = crate::solver::SolverLike::solution(& mut solver);

    let mut nexts = 0;
    let mut locates = 0;
    let mut settles = 0;
    let mut negates = 0;
    let mut selects = 0;
    let mut coerces = 0;
    let mut reverts = 0;
    
    let instant = std::time::Instant::now();

    while let Ok(action) = unsafe { solution.step() }
    {
        match action
        {
            crate::coerce::action::Action::Assert { action } =>
            {
                match action
                {
                    crate::assert::action::Action::Next =>
                    {
                        nexts += 1;
                    },
                    crate::assert::action::Action::Locate =>
                    {
                        locates += 1;
                    },
                    crate::assert::action::Action::Settle =>
                    {
                        settles += 1;
                    },
                    crate::assert::action::Action::Negate =>
                    {
                        negates += 1;
                    }
                };
            },
            crate::coerce::action::Action::Select =>
            {
                selects += 1;
            },
            crate::coerce::action::Action::Coerce =>
            {
                if instant.elapsed() > timeout
                {
                    return None;
                };

                coerces += 1;
            },
            crate::coerce::action::Action::Revert =>
            {
                reverts += 1;
            }
        };
    };

    let duration = instant.elapsed();

    let report = GaugeReport { nexts, locates, settles, negates, selects, coerces, reverts, duration };

    return Some(report);
}

const PERPETUAL: std::time::Duration = std::time::Duration::from_secs(u64::MAX);

#[derive(Debug, Clone)]
pub struct BenchReport
{
    pub sample: GaugeReport,
    pub durations: Box<[std::time::Duration]>
}

pub fn bench<Model, Audit, Field, Queue, Cache, Probe>(model: Model, minimum: usize, prepare: std::time::Duration, mut timeout: std::time::Duration) -> BenchReport
where
    Model: crate::model::ModelLike + Copy,
    Audit: crate::assert::audit::AuditLike<Model, Field> + crate::coerce::revert::Revertible,
    Field: crate::assert::field::FieldLike<Model>,
    Queue: crate::assert::queue::QueueLike<Model>,
    Cache: crate::assert::cache::CacheLike<Model>,
    Probe: crate::coerce::probe::ProbeLike<Model, Field> + crate::coerce::revert::Revertible,
    crate::coerce::solver::Solver<Model, Audit, Field, Queue, Cache, Probe>: From<Model>
{
    timeout += prepare;

    let instant = std::time::Instant::now();

    let mut durations = Vec::new();

    let sample = gauge(model, PERPETUAL).unwrap();

    loop
    {
        if durations.len() >= minimum && instant.elapsed() > timeout
        {
            break;
        };

        let report = solve(model);

        if instant.elapsed() < prepare
        {
            continue;
        };

        durations.push(report.duration);

        print!("{:0.0?} ", report.duration);

        std::io::Write::flush(& mut std::io::stdout()).unwrap();
    };

    println!("");

    let durations = durations.into_boxed_slice();

    let report = BenchReport { sample, durations };

    return report;
}
