use crate::bindings::*;


/**
Representation of a prolog module. It give access to queries.
*/
#[derive(Copy,Clone,Debug)]
pub struct Module<'a>(ModuleT<'a>);
impl<'a> Module<'a>
{
    pub(crate) fn new(module: ModuleT<'a>)->Self {Self(module)}

    //pub(crate) fn default_module()->Self {Self(context())}
    //pub(crate) fn get<'a>(name: impl Into<&'a str>)->Self {Self(new_module(name))}

    //pub(crate) fn default_module()->Self {Self(std::ptr::null())}
    //pub(crate) fn get<'a>(name: impl Into<&'a str>)->Self {Self(std::ptr::null())}


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

//Having Arc<Swipl> make it deinitialize after every module, making it safe without the inconvenience of reference lifetime.
unsafe impl<'a> Send for Module<'a> {}
unsafe impl<'a> Sync for Module<'a> {}
