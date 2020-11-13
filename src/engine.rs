use crate::bindings::*;

use crate::frame::Frame;

use crate::swi_prolog::ENGINE;

#[cfg(log)]
use std::sync::atomic::{AtomicU32,Ordering};
#[cfg(log)]
static ENGINE_COUNTER: AtomicU32 = AtomicU32::new(0);
#[cfg(log)]
static ENGINE_REFS_COUNTER: AtomicU32 = AtomicU32::new(0);


pub struct Engine
{
    handle: EngineT,
}
impl Engine
{
    pub fn new()->Self
    {
        let engine = Self
        {
            handle: create_engine(),
        };
        #[cfg(log)]
        log::trace!(target:"Engine created: {:?}", ENGINE_COUNTER.fetch_add(1,Ordering::Relaxed));

        engine
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        #[cfg(log)]
        log::trace!(target:"Engine destroyed: {:?}", ENGINE_COUNTER.fetch_add(1,Ordering::Relaxed));

        destroy_engine(self.handle);
    }
}
impl std::ops::Deref for Engine {
    type Target = EngineT;
    fn deref(&self) -> &Self::Target {&self.handle}
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}



pub struct EngineRef<'a>(Option<Engine>,Option<Frame<'a>>);
impl<'a> EngineRef<'a>
{
    pub fn new(engine: Engine)->Self {
        set_engine(*engine);
        #[cfg(log)]
        log::trace!("Engine ref created: {:?}", ENGINE_REFS_COUNTER.fetch_add(1,Ordering::Relaxed));

        Self(Some(engine),Some(Frame::new()))
    }

    pub fn get_module(&self,name: Option<String>)->Module<'a>{
        match name
        {
            Some(name)=>Module::new(new_module(name.as_str())),
            None=>Module::new(context())
        }
    }
    pub fn get_frame(&self)->&Frame {self.1.as_ref().unwrap()}
}
impl<'a> Drop for EngineRef<'a> {
    fn drop(&mut self)
    {
        {self.1.take().unwrap();}
        #[cfg(log)]
        log::trace!("Engine ref destroyed: {:?}", ENGINE_REFS_COUNTER.fetch_add(1,Ordering::Relaxed));

        set_engine(std::ptr::null());
        ENGINE.put_engine(self.0.take().unwrap());
    }
}
impl<'a> std::ops::Deref for EngineRef<'a> {
    type Target = Engine;
    fn deref(&self) -> &Self::Target {&self.0.as_ref().unwrap()}
}


