
pub trait Revertible
{
    fn save(& mut self);
    
    fn load(& mut self) -> bool;
}