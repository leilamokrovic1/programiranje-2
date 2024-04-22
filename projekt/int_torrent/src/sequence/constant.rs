use crate::sequence::models::Sequence;


//Popravi
pub struct Constant<T> {
    value: T,
}


impl<T> Sequence<T> for Constant<T>
where
    T: Copy, 
{
    
    fn k_th(&self, _n: usize) -> Option<T> {
        Some(self.value) 
    }
}