
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action
{
    Assert { action: crate::assert::action::Action },
    Select,
    Coerce,
    Revert
}

impl Default for Action
{
    fn default() -> Self 
    {
        return Self::Assert { action: Default::default() };
    }
}