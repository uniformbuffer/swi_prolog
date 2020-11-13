use crate::bindings::*;


/**
Representation of a prolog module. It give access to queries.
*/
#[derive(Copy,Clone,Debug)]
pub struct Module<'a>(ModuleT<'a>);
impl<'a> Module<'a>
{
    pub(crate) fn new(module: ModuleT<'a>)->Self {Self(module)}

    /**
    Get the name of the module.
    */
    pub fn name(&self)->String
    {
        module_name(**self)
    }
}

impl<'a> std::ops::Deref for Module<'a> {
    type Target = ModuleT<'a>;
    fn deref(&self) -> &Self::Target {&self.0}
}
