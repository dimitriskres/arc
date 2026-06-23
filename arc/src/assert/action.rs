
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action
{
    Next,
    Locate,
    Settle,
    Negate
}

impl Default for Action
{
    fn default() -> Self
    {
        return Self::Next;
    }
}