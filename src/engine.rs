use crate::bindings::*;

use crate::frame::Frame;
use std::rc::Rc;

use crate::swi_prolog::ENGINE;

pub struct Engine
{
    handle: EngineT,
    frame: Rc<Frame>
}
impl Engine
{
    pub fn new()->Self
    {
        let engine = Self
        {
            handle: create_engine(),
            frame: Frame::new(None)
        };
        engine
    }

    pub fn get_frame(&self)->Rc<Frame> {self.frame.nest().unwrap()}
}

impl Drop for Engine {
    fn drop(&mut self) {
        destroy_engine(self.handle);
    }
}
impl std::ops::Deref for Engine {
    type Target = EngineT;
    fn deref(&self) -> &Self::Target {&self.handle}
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}



pub struct EngineRef(Option<Engine>);
impl EngineRef
{
    pub fn new(engine: Engine)->Self {
        set_engine(*engine);
        Self(Some(engine))
    }

    pub fn get_module<'a>(&self,name: Option<String>)->Module<'a>{
        match name
        {
            Some(name)=>Module::new(new_module(name.as_str())),
            None=>Module::new(context())
        }

    }
}
impl Drop for EngineRef {
    fn drop(&mut self)
    {
        let engine = self.0.take().unwrap();
        set_engine(std::ptr::null());
        ENGINE.put_engine(engine);
    }
}
impl std::ops::Deref for EngineRef {
    type Target = Engine;
    fn deref(&self) -> &Self::Target {&self.0.as_ref().unwrap()}
}


