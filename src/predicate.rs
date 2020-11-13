use crate::bindings::*;

#[derive(Clone)]
pub struct Predicate(PredicateT);
impl Predicate
{
    pub fn new<'a>(module: &Module,name: impl Into<&'a str>,ariety: usize)->Self
    {
        let functor = Functor::new(name,ariety);
        Self(pred(*functor,**module))
    }
    /*
    pub fn new_from_current<'a>(name: impl Into<&'a str>,ariety: usize)->Self
    {
        let functor = Functor::new(name,ariety);
        Self(pred(&functor,&Module::current()))
    }
    */
    //pub fn name(&self)->String {self.name.clone()}
    //pub fn ariety(&self)->usize {self.ariety.clone()}
}

impl std::ops::Deref for Predicate {
    type Target = PredicateT;
    fn deref(&self) -> &Self::Target {&self.0}
}
/*
impl<'a,N: Into<&'a str>> From<(N ,usize)> for Predicate
{
    fn from(name_and_ariety: (N,usize)) -> Self {
        let (name,ariety) = name_and_ariety;
        Self::new_from_current(name,ariety)
    }
}

impl From<Functor> for Predicate
{
    fn from(functor: Functor) -> Self {
        Self(pred(&functor,&Module::current()))
    }
}
*/
