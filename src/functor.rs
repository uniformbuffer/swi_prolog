use crate::bindings::*;

#[derive(Clone,Debug)]
pub struct Functor(FunctorT);
impl Functor
{
    pub fn new<'a>(name: impl Into<&'a str>,ariety: usize)->Self
    {
        Self(new_functor(name,ariety))
    }

    pub fn from_raw(functor: FunctorT)->Self
    {
        Self(functor)
    }

    pub fn name(&self)->String {functor_name(self.0)}
    pub fn ariety(&self)->usize {functor_arity(self.0)}
}

impl std::ops::Deref for Functor {
    type Target = FunctorT;
    fn deref(&self) -> &Self::Target {&self.0}
}


